use std::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use mapkit::prelude::*;

mod common;

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

extern "C" {
    fn pthread_main_np() -> i32;
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
    map_view.set_shows_user_location(true).unwrap();
    assert!(map_view.shows_user_location().unwrap());
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
        assert!(
            matches!(error, MapKitError::MainThreadRequired(_)),
            "{error:?}"
        );
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

fn completer_state_round_trips_on_the_main_thread() {
    let completer = MKLocalSearchCompleter::new().unwrap();
    let region = MKCoordinateRegion::with_distance(apple_park(), 2_000.0, 2_000.0).unwrap();
    let result_types = MKLocalSearchCompleterResultType::ADDRESS
        | MKLocalSearchCompleterResultType::POINT_OF_INTEREST;
    completer.set_region(region).unwrap();
    completer.set_result_types(result_types).unwrap();

    assert_eq!(completer.query_fragment().unwrap(), "");
    assert_eq!(completer.result_types().unwrap(), result_types);
    assert!((completer.region().unwrap().center.latitude - apple_park().latitude).abs() < 1e-6);
    assert!(completer.results().unwrap().is_empty());
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

struct CompleterRecorder {
    log: Arc<Log>,
    name: &'static str,
}

impl CompleterRecorder {
    fn new(log: &Arc<Log>, name: &'static str) -> Self {
        Self {
            log: Arc::clone(log),
            name,
        }
    }
}

impl Drop for CompleterRecorder {
    fn drop(&mut self) {
        self.log.drops.fetch_add(1, Ordering::SeqCst);
    }
}

impl MKLocalSearchCompleterDelegate for CompleterRecorder {
    fn completer_did_update_results(
        &mut self,
        completer: &MKLocalSearchCompleter,
        results: &[MKLocalSearchCompletion],
    ) {
        self.log.push(format!(
            "{} update main={} query={} same={} count={}",
            self.name,
            unsafe { pthread_main_np() } != 0,
            completer.query_fragment().unwrap(),
            completer.results().unwrap() == results,
            results.len()
        ));
    }

    fn completer_did_fail_with_error(
        &mut self,
        completer: &MKLocalSearchCompleter,
        error: &MapKitError,
    ) {
        self.log.push(format!(
            "{} fail main={} query={} framework={}: {error}",
            self.name,
            unsafe { pthread_main_np() } != 0,
            completer.query_fragment().unwrap(),
            matches!(error, MapKitError::Framework(_))
        ));
    }
}

fn completer_delegates_are_replaced_and_released() {
    let log = Arc::new(Log::default());
    let completer = MKLocalSearchCompleter::new().unwrap();
    let second = autoreleasing(|| {
        let first = completer
            .set_delegate(CompleterRecorder::new(&log, "first"))
            .unwrap();
        let second = completer
            .set_delegate(CompleterRecorder::new(&log, "second"))
            .unwrap();
        drop(first);
        second
    });
    assert_eq!(log.drops.load(Ordering::SeqCst), 1);

    autoreleasing(|| {
        drop(completer);
        drop(second);
    });
    assert_eq!(log.drops.load(Ordering::SeqCst), 2);
    assert!(log.take().is_empty());
}

fn completer_delegate_receives_results_on_the_main_thread() {
    let log = Arc::new(Log::default());
    let completer = MKLocalSearchCompleter::new().unwrap();
    let registration = autoreleasing(|| {
        let registration = completer
            .set_delegate(CompleterRecorder::new(&log, "live"))
            .unwrap();
        let region = MKCoordinateRegion::with_distance(apple_park(), 5_000.0, 5_000.0).unwrap();
        completer.set_region(region).unwrap();
        completer.set_query_fragment("coffee").unwrap();
        registration
    });

    let deadline = Instant::now() + Duration::from_secs(20);
    let mut events = Vec::new();
    while events.is_empty() && Instant::now() < deadline {
        autoreleasing(|| pump_main_run_loop(5));
        events = log.take();
    }
    completer.cancel();
    autoreleasing(|| drop(registration));
    assert_eq!(log.drops.load(Ordering::SeqCst), 1);

    let event = events
        .first()
        .expect("MapKit delivered no completer result within 20 seconds");
    assert!(
        event.starts_with("live update main=true query=coffee same=true ")
            || event.starts_with("live fail main=true query=coffee framework=true: "),
        "{events:?}"
    );
}

#[cfg(feature = "async")]
fn async_local_search_resolves() {
    let worker = thread::spawn(|| {
        let request = MKLocalSearchRequest::new("Cupertino")
            .with_result_types(MKLocalSearchResultType::ADDRESS);
        let future = mapkit::async_api::AsyncMKLocalSearch::search(&request).expect("build");
        pollster::block_on(future)
    });
    let deadline = Instant::now() + Duration::from_secs(30);
    while !worker.is_finished() && Instant::now() < deadline {
        autoreleasing(|| pump_main_run_loop(5));
    }
    assert!(
        worker.is_finished(),
        "the local search did not resolve within 30 seconds"
    );
    let response = worker
        .join()
        .expect("worker thread")
        .expect("search should succeed");
    assert!(
        !response.map_items.is_empty(),
        "expected at least one result"
    );
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
    let offline: Vec<(&str, fn())> = vec![
        ("map_view_round_trip", map_view_round_trip),
        ("annotation_views_build", annotation_views_build),
        ("views_refuse_other_threads", views_refuse_other_threads),
        (
            "synchronous_waits_refuse_the_main_thread",
            synchronous_waits_refuse_the_main_thread,
        ),
        (
            "completer_state_round_trips_on_the_main_thread",
            completer_state_round_trips_on_the_main_thread,
        ),
        (
            "completer_delegates_are_replaced_and_released",
            completer_delegates_are_replaced_and_released,
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
    #[cfg_attr(not(feature = "async"), allow(unused_mut))]
    let mut live: Vec<(&str, fn())> = vec![
        (
            "user_tracking_button_updates_map_view",
            user_tracking_button_updates_map_view,
        ),
        (
            "delegate_receives_region_changes_and_supplies_renderers",
            delegate_receives_region_changes_and_supplies_renderers,
        ),
        (
            "completer_delegate_receives_results_on_the_main_thread",
            completer_delegate_receives_results_on_the_main_thread,
        ),
    ];
    #[cfg(feature = "async")]
    live.push(("async_local_search_resolves", async_local_search_resolves));

    for (name, check) in &offline {
        check();
        println!("test {name} ... ok");
    }
    let run_live = common::live_tests_enabled();
    for (name, check) in &live {
        if run_live {
            check();
            println!("test {name} ... ok");
        } else {
            println!("test {name} ... skipped");
        }
    }
    let (passed, skipped) = if run_live {
        (offline.len() + live.len(), 0)
    } else {
        (offline.len(), live.len())
    };
    println!("test result: ok. {passed} passed; 0 failed; {skipped} skipped");
}
