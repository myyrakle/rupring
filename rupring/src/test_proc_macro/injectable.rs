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

        let mut di_context = rupring::DIContext::new();
        di_context.register(counter_service_factory.provide(&di_context));

        let service = di_context.get::<CounterService>().unwrap();

        assert_eq!(service.get(), 0, "초기 카운트 0",);

        service.increment();

        assert_eq!(service.get(), 1, "카운트 1",);
    }
}

mod test_1대1_inject {
    use crate::{self as rupring, IProvider};

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
    fn test_1대1_inject() {
        impl Clone for SomeRepositoryFactory {
            fn clone(&self) -> Self {
                SomeRepositoryFactory {}
            }
        }

        impl Clone for SomeServiceFactory {
            fn clone(&self) -> Self {
                SomeServiceFactory {}
            }
        }

        let some_repository_factory = SomeRepositoryFactory {};
        let some_service_factory = SomeServiceFactory {};

        assert_eq!(
            vec![std::any::TypeId::of::<SomeRepository>()],
            some_service_factory.dependencies(),
            "SomeService는 종속성으로 SomeRepository를 가짐",
        );

        assert_eq!(
            Vec::<std::any::TypeId>::new(),
            some_repository_factory.dependencies(),
            "SomeRepository는 종속성 없음",
        );

        let mut di_context = rupring::DIContext::new();
        di_context.register(some_repository_factory.provide(&di_context));
        di_context.register(some_service_factory.provide(&di_context));

        let repository = some_repository_factory
            .provide(&di_context)
            .downcast::<SomeRepository>()
            .unwrap();

        assert_eq!(repository.some_value, 0, "SomeRepository 초기값 0",);

        let service = some_service_factory
            .provide(&di_context)
            .downcast::<SomeService>()
            .unwrap();

        assert_eq!(
            service.some_repository.some_value, 0,
            "SomeService::SomeRepository 초기값 0",
        );
    }
}

mod test_1대1_inject_module_내_테스트 {
    use crate::{self as rupring, IProvider};

    mod foo {
        use crate::{self as rupring};

        #[derive(Debug, Clone, Default)]
        pub struct SomeRepository {
            pub some_value: i32,
        }

        #[rupring_macro::Injectable(SomeRepositoryFactory)]
        fn inject_some_repository() -> SomeRepository {
            SomeRepository::default()
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct SomeService {
        pub some_repository: foo::SomeRepository,
    }

    #[rupring_macro::Injectable(SomeServiceFactory)]
    fn inject_some_service(some_repository: foo::SomeRepository) -> SomeService {
        SomeService { some_repository }
    }

    #[test]
    fn test_1대1_inject() {
        impl Clone for foo::SomeRepositoryFactory {
            fn clone(&self) -> Self {
                foo::SomeRepositoryFactory {}
            }
        }

        impl Clone for SomeServiceFactory {
            fn clone(&self) -> Self {
                SomeServiceFactory {}
            }
        }

        let some_repository_factory = foo::SomeRepositoryFactory {};
        let some_service_factory = SomeServiceFactory {};

        assert_eq!(
            vec![std::any::TypeId::of::<foo::SomeRepository>()],
            some_service_factory.dependencies(),
            "SomeService는 종속성으로 SomeRepository를 가짐",
        );

        assert_eq!(
            Vec::<std::any::TypeId>::new(),
            some_repository_factory.dependencies(),
            "SomeRepository는 종속성 없음",
        );

        let mut di_context = rupring::DIContext::new();
        di_context.register(some_repository_factory.provide(&di_context));
        di_context.register(some_service_factory.provide(&di_context));

        let repository = some_repository_factory
            .provide(&di_context)
            .downcast::<foo::SomeRepository>()
            .unwrap();

        assert_eq!(repository.some_value, 0, "SomeRepository 초기값 0",);

        let service = some_service_factory
            .provide(&di_context)
            .downcast::<SomeService>()
            .unwrap();

        assert_eq!(
            service.some_repository.some_value, 0,
            "SomeService::SomeRepository 초기값 0",
        );
    }
}
