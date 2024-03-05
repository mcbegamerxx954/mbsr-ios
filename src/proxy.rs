use objc2::{extern_class, extern_methods, mutability, ClassType};
use objc2::runtime::NSObject;
use objc2::rc::Id;
use icrate::Foundation::{NSCoding, NSCopying, NSSecureCoding, NSURL, NSString};
/*
* This module contains some extern classes to access
* Private ios functions and classes that we need to 
* do stuff.
*/
extern_class!(
    #[derive(PartialEq, Eq, Hash)] // Uses the superclass' implementation
    // Specify the class and struct name to be used
    pub struct LSBundleProxy;

    // Specify the superclass, in this case `NSObject`
    unsafe impl ClassType for LSBundleProxy {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        // Optionally, specify the name of the class, if it differs from
        // the struct name.
        // const NAME: &'static str = "NSFormatter";
    }
);
extern_methods!(
    unsafe impl LSBundleProxy {
        #[method_id(bundleURL)]
        pub unsafe fn bundleUrl(&self) -> Id<NSURL>;
        
        #[method_id(dataContainerUrl)]
        pub unsafe fn dataContainerUrl(&self) -> Id<NSURL>;

        #[method_id(bundleProxyForIdentifier:)]
        pub unsafe fn bundleProxyForIdentifier(identifier: &NSString) -> Id<Self>;
    }
);
unsafe impl NSCopying for LSBundleProxy {}
unsafe impl NSSecureCoding for LSBundleProxy {}
unsafe impl NSCoding for LSBundleProxy {}

