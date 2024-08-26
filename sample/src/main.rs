use kabto::tag::*;

fn app() -> impl kabto::Component {
    //div.abbr("")
    div
}

fn main() {
    fn type_params() {
        struct S<T>(T);
        
        impl S<usize> {
            fn a(&self) {}
        }
        
        impl S<String> {
            fn b(&self) {}
        }
        
        impl<T> S<T> {
            fn c(&self) {}
        }
        
        let v: S<()> = S(());
        v.c();
    }
    fn const_params() {
        struct S<const C: usize>;
        
        impl S<0> {
            fn a(&self) {}
        }
        
        impl S<1> {
            fn b(&self) {}
        }
        
        impl<const C: usize> S<C> {
            fn c(&self) {}
        }
        
        let v: S<0> = S;
        v.a();
    }
}
