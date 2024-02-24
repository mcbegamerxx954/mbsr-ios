#import "rust_bridge.h"
%hook NSBundle
- (NSString*)pathForResource:(NSString*)path ofType:(NSString*)type {
  NSString* original = %orig;
  NSString* rust_rep_result = mbsr_get_rep_path(path, type);
  // Check
  if ([rust_rep_result isEqualToString:@""]) {
    return original;
  }
  return rust_rep_result;

}

- (NSString*)pathForResource:(NSString*)path ofType:(NSString*)type inDirectory:(NSString*)directory {
  NSString* original = %orig;
  NSString* rust_rep_result = mbsr_get_rep_path(path, type);
  // Check
  if ([rust_rep_result isEqualToString:@""]) {
    return original;
  }
  return rust_rep_result;

}

%end
%ctor {
  mbsr_ruststartup();
  %init;
}
/* How to Hook with Logos
Hooks are written with syntax similar to that of an Objective-C @implementation.
You don't need to #include <substrate.h>, it will be done automatically, as will
the generation of a class list and an automatic constructor.

%hook ClassName

// Hooking a class method
+ (id)sharedInstance {
	return %orig;
}

// Hooking an instance method with an argument.
- (void)messageName:(int)argument {
	%log; // Write a message about this call, including its class, name and arguments, to the system log.

	%orig; // Call through to the original function with its original arguments.
	%orig(nil); // Call through to the original function with a custom argument.

	// If you use %orig(), you MUST supply all arguments (except for self and _cmd, the automatically generated ones.)
}

// Hooking an instance method with no arguments.
- (id)noArguments {
	%log;
	id awesome = %orig;
	[awesome doSomethingElse];

	return awesome;
}

// Always make sure you clean up after yourself; Not doing so could have grave consequences!
%end
*/


