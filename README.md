# Guest Resource

This repository demonstrates we cannot use guest defined resource in export functions.

The generated codes snippet by `wit_bindgen::generate!` go as 

```rust
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod helloworld {
        #[allow(dead_code)]
        pub mod example {
            #[allow(dead_code, clippy::all)]
            pub mod greeter_api {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct Greeter {
                    handle: _rt::Resource<Greeter>,
                }

                // ...

                pub trait GuestGreeter: 'static {
                  // ..
                }


                // ...
            }

            #[allow(dead_code, clippy::all)]
            pub mod api {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Greeter =
                    super::super::super::super::exports::helloworld::example::greeter_api::Greeter;

                // ...
                
                pub trait Guest {
                    fn hello_world(g: Greeter);

                    fn hi(who: _rt::String);
                }

                // ...
            }
        }
    }
}
```

The greeter used by `api::Guest::hello_world()` is the `greeter_api::Greeter` struct, but not my expected `greeter_api::GuestGreeter` trait.

Really appreciate if someone could help me out~
