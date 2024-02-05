macro_rules! renderer_fn {
    (
        $name:ident<$($ty:ident),*>($($arg_name:ident: $arg_ty:ty),*)
        where $($clause:tt)*
    ) => {
        #[derive(Clone)]
        pub struct $name<$($ty),*> (
            Rc<dyn Fn($($arg_ty),*) -> View>,
        )
        where $($clause)*;

        impl<F, Ret, $($ty),*> From<F> for $name<$($ty),*>
        where
            F: Fn($($arg_ty),*) -> Ret + 'static,
            Ret: IntoView,
            $($clause)*
        {
            fn from(f: F) -> Self {
                Self(Rc::new(move |$($arg_name),*| {
                    f($($arg_name),*).into_view()
                }))
            }
        }

        impl<$($ty),*> $name <$($ty),*>
        where $($clause)*
        {
            pub fn run(&self, $($arg_name: $arg_ty),*) -> View {
                (self.0)($($arg_name),*)
            }
        }
    };

    (
        $name:ident<$($ty:ident),*>($($arg_name:ident: $arg_ty:ty),*)
        default $default:ident
        where $($clause:tt)*
    ) => {
        renderer_fn!(
            $name<$($ty),*>($($arg_name: $arg_ty),*)
            where $($clause)*
        );

        impl<$($ty),*> Default for $name<$($ty),*>
        where $($clause)*
        {
            fn default() -> Self {
                Self(Rc::new(move |$($arg_name),*| {
                    $default($($arg_name),*).into_view()
                }))
            }
        }
    };

    (
        $name:ident<$($ty:ident),*>($($arg_name:ident: $arg_ty:ty),*)
        default $default:ident
    ) => {
        renderer_fn!(
            $name<$($ty),*>($($arg_name: $arg_ty),*)
            default $default
            where
        );
    };

    (
        $name:ident($($arg_name:ident: $arg_ty:ty),*)
        default $default:ident
    ) => {
        renderer_fn!(
            $name<>($($arg_name: $arg_ty),*)
            default $default
        );
    };

    (
        $name:ident<$($ty:ident),*>($($arg_name:ident: $arg_ty:ty),*)
    ) => {
        renderer_fn!(
            $name<$($ty),*>($($arg_name: $arg_ty),*)
            where
        );
    };

    (
        $name:ident($($arg_name:ident: $arg_ty:ty),*)
    ) => {
        renderer_fn!(
            $name<>($($arg_name: $arg_ty),*)
            where
        );
    };
}

pub(crate) use renderer_fn;
