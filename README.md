# INCORRECT example showing the need to deal with Windows execution policies

On Windows, the default execution policy is `Restricted`. This means that
by default even [simple Rust programs](https://github.com/JohnScience/neon_example_cant_find/blob/main/pure_rust_version/src/main.rs) cannot be executed.

![screenshot](https://i.imgur.com/XVCkgCS.png)

In answers for ["How do I set the Windows execution level to ask user for administrator privileges for a Rust program?"](https://stackoverflow.com/questions/53846395/how-do-i-set-the-windows-execution-level-to-ask-user-for-administrator-privilege) question on Stack Overflow, the solution was to use [`winres`](https://crates.io/crates/winres) library when targeting Windows.

However, this doesn't work for [`neon`](https://crates.io/crates/neon) modules for Node.js, because they are not executables.

## Open questions

How does Node.js deal with this problem? How should [`neon`](https://crates.io/crates/neon) modules' authors deal with this problem?

## Related issues

* [neon-bindings/neon/issues/"Neon modules for Electron apps targeting Windows may require privileges"](https://github.com/neon-bindings/neon/issues/956)

## Why is this example incorrect?

As <https://github.com/kjvalencik> pointed out in the related issue, the script shouldn't have worked in the first place because it didn't properly handle the case where the <app_data_dir>/<APP_NAME> does not exist. The step with creation of the directory was omitted. After the fix, it worked properly even without permission elevation.
