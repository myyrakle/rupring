mod test_단일_inject {
    use crate::{self as rupring, IProvider};
    use std::{
        any::TypeId,
        sync::{Arc, Mutex},
    };

    #[derive(Debug, Clone, Default)]
    pub struct CounterService {
        pub counter: Arc<Mutex<i32>>,
    }

    impl PartialEq for CounterService {
        fn eq(&self, other: &Self) -> bool {
            Arc::ptr_eq(&self.counter, &other.counter)
        }
    }

    impl CounterService {
        pub fn new() -> Self {
            CounterService {
                counter: Arc::new(Mutex::new(0)),
            }
        }

        pub fn increment(&self) {
            let mut counter = self.counter.lock().unwrap();
            *counter += 1;
        }

        pub fn get(&self) -> i32 {
            let counter = self.counter.lock().unwrap();
            *counter
        }
    }

    #[rupring_macro::Injectable(CounterServiceFactory)]
    fn inject_counter_service() -> CounterService {
        CounterService::new()
    }

    #[test]
    fn test_단일_inject() {
        let counter_service_factory = CounterServiceFactory {};
        assert_eq!(
            Vec::<TypeId>::new(),
            counter_service_factory.dependencies(),
            "종속성 없음",
        );

        let di_context = rupring::DIContext::new();

        let service = counter_service_factory
            .provide(&di_context)
            .downcast::<CounterService>()
            .unwrap();

        assert_eq!(service.get(), 0, "초기 카운트 0",);

        service.increment();

        assert_eq!(service.get(), 1, "카운트 1",);
    }
}

mod test_1대1_inject {

    use crate::{self as rupring};

    #[derive(Debug, Clone, Default)]
    pub struct SomeRepository {
        pub some_value: i32,
    }

    #[derive(Debug, Clone, Default)]
    pub struct SomeService {
        pub some_repository: SomeRepository,
    }

    #[rupring_macro::Injectable(SomeRepositoryFactory)]
    fn inject_some_repository() -> SomeRepository {
        SomeRepository::default()
    }

    #[rupring_macro::Injectable(SomeServiceFactory)]
    fn inject_some_service(some_repository: SomeRepository) -> SomeService {
        SomeService { some_repository }
    }

    #[test]
    fn test_1대1_inject() {}
}
