use std::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use mapkit::prelude::*;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFRunLoopDefaultMode: *const c_void;
    fn CFRunLoopRunInMode(mode: *const c_void, seconds: f64, return_after: u8) -> i32;
}

#[link(name = "objc")]
extern "C" {
    fn objc_autoreleasePoolPush() -> *mut c_void;
    fn objc_autoreleasePoolPop(pool: *mut c_void);
}

fn autoreleasing<R>(body: impl FnOnce() -> R) -> R {
    let pool = unsafe { objc_autoreleasePoolPush() };
    let result = body();
    unsafe { objc_autoreleasePoolPop(pool) };
    result
}

fn pump_main_run_loop(iterations: usize) {
    for _ in 0..iterations {
        unsafe { CFRunLoopRunInMode(kCFRunLoopDefaultMode, 0.02, 0) };
    }
}

fn pump_main_run_loop_until_done<T>(worker: thread::JoinHandle<T>) -> T {
    while !worker.is_finished() {
        unsafe { CFRunLoopRunInMode(kCFRunLoopDefaultMode, 0.02, 0) };
    }
    worker.join().expect("worker thread")
}

const fn apple_park() -> MKCoordinate {
    MKCoordinate::new(37.3349, -122.0090)
}

fn map_view_round_trip() {
    let map_view = MKMapView::new(MKScreenSize::new(320.0, 200.0)).unwrap();
    let center = apple_park();
    let region = MKCoordinateRegion::with_distance(center, 1_000.0, 1_000.0).unwrap();
    let camera = MKMapCamera::looking_at_center_coordinate(center, 750.0, 35.0, 90.0);
    let zoom_range = MKMapCameraZoomRange::with_min_center_coordinate_distance(250.0);
    let configuration: MKMapConfiguration = MKStandardMapConfiguration::new()
        .with_elevation_style(MKMapElevationStyle::Realistic)
        .with_shows_traffic(true)
        .into();

    map_view.set_region(region, false).unwrap();
    map_view.set_camera(camera, false).unwrap();
    map_view
        .set_camera_zoom_range(Some(zoom_range), false)
        .unwrap();
    map_view
        .set_camera_boundary(
            Some(MKMapCameraBoundary::from_region(region).unwrap()),
            false,
        )
        .unwrap();
    map_view.set_preferred_configuration(configuration).unwrap();

    assert!((map_view.region().unwrap().center.latitude - center.latitude).abs() < 1e-6);
    assert!(
        (map_view.camera().unwrap().center_coordinate.longitude - center.longitude).abs() < 1e-6
    );
    assert_eq!(
        map_view.preferred_configuration().unwrap().unwrap().kind,
        MKMapConfigurationKind::Standard
    );
    assert!(map_view
        .convert_coordinate_to_point(center)
        .unwrap()
        .x
        .is_finite());
}

fn annotation_views_build() {
    let coordinate = apple_park();
    let annotation = MKPointAnnotation::new(coordinate).unwrap();
    annotation.set_title(Some("Apple Park")).unwrap();

    let view = MKAnnotationView::new(Some(&annotation), Some("reuse-id")).unwrap();
    view.set_can_show_callout(true).unwrap();
    view.set_center_offset(MKScreenPoint::new(4.0, -8.0))
        .unwrap();
    assert_eq!(
        view.reuse_identifier().unwrap().as_deref(),
        Some("reuse-id")
    );
    assert_eq!(
        view.annotation_title().unwrap().as_deref(),
        Some("Apple Park")
    );
    assert!(!MKAnnotationView::callout_info_did_change_notification()
        .unwrap()
        .is_empty());

    let marker = MKMarkerAnnotationView::new(Some(&annotation), Some("marker-id")).unwrap();
    marker.set_glyph_text(Some("A")).unwrap();
    marker.set_animates_when_added(true).unwrap();
    assert_eq!(marker.glyph_text().unwrap().as_deref(), Some("A"));

    let map_item = MKMapItem::new(MKPlacemark::new(coordinate)).with_name("Apple Park");
    let map_item_annotation = MKMapItemAnnotation::new(&map_item).unwrap();
    let pin = MKPinAnnotationView::new(Some(&map_item_annotation), Some("pin-id")).unwrap();
    pin.set_animates_drop(true).unwrap();
    pin.set_pin_color(MKPinAnnotationColor::Purple).unwrap();
    assert_eq!(pin.pin_color().unwrap(), MKPinAnnotationColor::Purple);
    assert_eq!(
        pin.annotation_title().unwrap().as_deref(),
        Some("Apple Park")
    );

    let map_view = MKMapView::new(MKScreenSize::new(320.0, 200.0)).unwrap();
    map_view.add_annotation(&map_item_annotation).unwrap();
    map_view.set_shows_user_location(true).unwrap();
    let user_location = map_view.user_location().unwrap();
    let user_view = MKUserLocationView::new(Some(&user_location), Some("user-id")).unwrap();
    user_view.set_can_show_callout(true).unwrap();
    assert_eq!(
        user_view.reuse_identifier().unwrap().as_deref(),
        Some("user-id")
    );
}

fn user_tracking_button_updates_map_view() {
    let map_view = MKMapView::new(MKScreenSize::new(256.0, 256.0)).unwrap();
    let button = MKUserTrackingButton::new(&map_view);
    button.set_visible(true).unwrap();
    button
        .set_tracking_mode(MKUserTrackingMode::Follow, false)
        .unwrap();
    assert!(button.is_visible().unwrap());
    assert_eq!(button.tracking_mode().unwrap(), MKUserTrackingMode::Follow);
}

fn views_refuse_other_threads() {
    let results = thread::spawn(|| {
        let annotation = MKPointAnnotation::new(apple_park()).unwrap();
        [
            MKMapView::new(MKScreenSize::new(10.0, 10.0)).map(drop),
            MKAnnotationView::new(Some(&annotation), None).map(drop),
            MKMarkerAnnotationView::new(Some(&annotation), None).map(drop),
        ]
    })
    .join()
    .unwrap();
    for result in results {
        let error = result.expect_err("views must refuse to run off the main thread");
        assert!(error.to_string().contains("main thread"), "{error}");
    }
}

fn synchronous_waits_refuse_the_main_thread() {
    let search = MKLocalSearch::new(&MKLocalSearchRequest::new("coffee")).unwrap();
    let error = search
        .start()
        .map(drop)
        .expect_err("a synchronous search must not spin the main run loop");
    assert!(error.to_string().contains("main thread"), "{error}");
}

fn completer_accepts_an_unbounded_timeout() {
    let worker = thread::spawn(|| {
        let completer = MKLocalSearchCompleter::new().unwrap();
        completer.refresh_with_timeout(Duration::MAX)
    });
    let results = pump_main_run_loop_until_done(worker).expect("completer results");
    assert!(results.is_empty());
}

#[derive(Default)]
struct Log {
    events: Mutex<Vec<String>>,
    drops: AtomicUsize,
}

impl Log {
    fn push(&self, event: impl Into<String>) {
        self.events.lock().unwrap().push(event.into());
    }

    fn take(&self) -> Vec<String> {
        std::mem::take(&mut *self.events.lock().unwrap())
    }
}

struct RecordingDelegate {
    log: Arc<Log>,
    name: &'static str,
    recenter_on_change: bool,
    panic_on_first_change: bool,
}

impl RecordingDelegate {
    fn new(log: &Arc<Log>, name: &'static str) -> Self {
        Self {
            log: Arc::clone(log),
            name,
            recenter_on_change: false,
            panic_on_first_change: false,
        }
    }
}

impl Drop for RecordingDelegate {
    fn drop(&mut self) {
        self.log.drops.fetch_add(1, Ordering::SeqCst);
    }
}

impl MKMapViewDelegate for RecordingDelegate {
    fn map_view_region_will_change_animated(&mut self, _map_view: &MKMapView, animated: bool) {
        self.log
            .push(format!("{} will_change {animated}", self.name));
    }

    fn map_view_region_did_change_animated(&mut self, map_view: &MKMapView, animated: bool) {
        self.log
            .push(format!("{} did_change {animated}", self.name));
        if self.panic_on_first_change {
            self.panic_on_first_change = false;
            panic!("delegate panic");
        }
        if self.recenter_on_change {
            self.recenter_on_change = false;
            let region = MKCoordinateRegion::with_distance(apple_park(), 900.0, 900.0).unwrap();
            map_view.set_region(region, false).unwrap();
        }
    }

    fn map_view_renderer_for_overlay(
        &mut self,
        _map_view: &MKMapView,
        overlay: &MKAnyOverlay,
    ) -> Option<Box<dyn MKOverlayRendering>> {
        match overlay {
            MKAnyOverlay::Polyline(polyline) => {
                self.log.push(format!("{} renderer polyline", self.name));
                let renderer = MKPolylineRenderer::new(polyline).ok()?;
                renderer.set_stroke_end(1.0).ok()?;
                Some(Box::new(renderer))
            }
            MKAnyOverlay::Circle(circle) => {
                self.log.push(format!("{} renderer circle", self.name));
                Some(Box::new(MKCircleRenderer::new(circle).ok()?))
            }
            _ => None,
        }
    }
}

fn change_region(map_view: &MKMapView, meters: f64) {
    let region = MKCoordinateRegion::with_distance(apple_park(), meters, meters).unwrap();
    map_view.set_region(region, false).unwrap();
}

fn delegate_receives_region_changes_and_supplies_renderers() {
    let log = Arc::new(Log::default());
    let map_view = MKMapView::new(MKScreenSize::new(320.0, 200.0)).unwrap();
    autoreleasing(|| {
        let registration = map_view
            .set_delegate(RecordingDelegate::new(&log, "a"))
            .unwrap();

        change_region(&map_view, 5_000.0);
        assert_eq!(log.take(), ["a will_change false", "a did_change false"]);

        let polyline = MKPolyline::new(&[apple_park(), MKCoordinate::new(37.34, -122.02)]).unwrap();
        map_view
            .add_overlay(&polyline, MKOverlayLevel::AboveRoads)
            .unwrap();
        let circle = MKCircle::new(apple_park(), 150.0).unwrap();
        map_view
            .add_overlay(&circle, MKOverlayLevel::AboveLabels)
            .unwrap();
        pump_main_run_loop(10);
        let events = log.take();
        assert!(
            events.contains(&"a renderer polyline".to_string()),
            "{events:?}"
        );
        assert!(
            events.contains(&"a renderer circle".to_string()),
            "{events:?}"
        );
        assert_eq!(map_view.overlay_count().unwrap(), 2);
        drop(registration);
    });
    assert_eq!(log.drops.load(Ordering::SeqCst), 1);

    autoreleasing(|| change_region(&map_view, 2_000.0));
    assert!(log.take().is_empty());
}

fn replacing_the_delegate_keeps_the_new_one() {
    let log = Arc::new(Log::default());
    let map_view = MKMapView::new(MKScreenSize::new(320.0, 200.0)).unwrap();
    let second = autoreleasing(|| {
        let first = map_view
            .set_delegate(RecordingDelegate::new(&log, "first"))
            .unwrap();
        let second = map_view
            .set_delegate(RecordingDelegate::new(&log, "second"))
            .unwrap();
        drop(first);
        second
    });
    assert_eq!(log.drops.load(Ordering::SeqCst), 1);

    autoreleasing(|| {
        change_region(&map_view, 3_000.0);
        drop(second);
    });
    assert_eq!(
        log.take(),
        ["second will_change false", "second did_change false"]
    );
    assert_eq!(log.drops.load(Ordering::SeqCst), 2);
}

fn reentrant_and_panicking_delegates_are_contained() {
    let log = Arc::new(Log::default());
    let map_view = MKMapView::new(MKScreenSize::new(320.0, 200.0)).unwrap();
    autoreleasing(|| {
        let mut delegate = RecordingDelegate::new(&log, "d");
        delegate.recenter_on_change = true;
        let registration = map_view.set_delegate(delegate).unwrap();
        change_region(&map_view, 4_000.0);
        drop(registration);
    });
    assert_eq!(log.take(), ["d will_change false", "d did_change false"]);
    assert_eq!(log.drops.load(Ordering::SeqCst), 1);

    autoreleasing(|| {
        let mut delegate = RecordingDelegate::new(&log, "p");
        delegate.panic_on_first_change = true;
        let registration = map_view.set_delegate(delegate).unwrap();
        change_region(&map_view, 6_000.0);
        change_region(&map_view, 7_000.0);
        drop(registration);
    });
    assert_eq!(
        log.take(),
        [
            "p will_change false",
            "p did_change false",
            "p will_change false",
            "p did_change false",
        ]
    );
    assert_eq!(log.drops.load(Ordering::SeqCst), 2);
}

fn main() {
    let checks: [(&str, fn()); 9] = [
        ("map_view_round_trip", map_view_round_trip),
        ("annotation_views_build", annotation_views_build),
        (
            "user_tracking_button_updates_map_view",
            user_tracking_button_updates_map_view,
        ),
        ("views_refuse_other_threads", views_refuse_other_threads),
        (
            "synchronous_waits_refuse_the_main_thread",
            synchronous_waits_refuse_the_main_thread,
        ),
        (
            "completer_accepts_an_unbounded_timeout",
            completer_accepts_an_unbounded_timeout,
        ),
        (
            "delegate_receives_region_changes_and_supplies_renderers",
            delegate_receives_region_changes_and_supplies_renderers,
        ),
        (
            "replacing_the_delegate_keeps_the_new_one",
            replacing_the_delegate_keeps_the_new_one,
        ),
        (
            "reentrant_and_panicking_delegates_are_contained",
            reentrant_and_panicking_delegates_are_contained,
        ),
    ];
    for (name, check) in checks {
        check();
        println!("test {name} ... ok");
    }
    println!("test result: ok. {} passed; 0 failed", checks.len());
}
