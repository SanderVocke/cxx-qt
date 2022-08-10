#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QString = cxx_qt_lib::QString;
    }

    #[derive(Default)]
    pub struct Data {
        number: i32,
        string: QString,
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MyObject;

    impl UpdateRequestHandler for cxx_qt::QObject<MyObject> {
        fn handle_update_request(self: Pin<&mut Self>) {
            println!("update")
        }
    }
}
