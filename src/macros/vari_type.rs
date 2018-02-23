
macro_rules! vari_type {
    ( $varitype:ident ) => {
        // {
        #[derive(Debug)]
        struct $varitype {
            data: VariData,
            avi_: Rc<RefCell<Vari>>,
        }

        impl $varitype {
            fn new(v: Real, avi: Rc<RefCell<Vari>>) -> $varitype {
                $varitype {
                    data: VariData::new(v),
                    avi_: avi.clone(),
                }
            }
        }
        // }
    };

    ( $varitype:ident, Real ) => {
        // {
        #[derive(Debug)]
        struct $varitype {
            data: VariData,
            avi_: Rc<RefCell<Vari>>,
            bd_: Real
        }

        impl $varitype {
            fn new(v: Real, avi: Rc<RefCell<Vari>>, b: Real) -> $varitype {
                $varitype {
                    data: VariData::new(v),
                    avi_: avi.clone(),
                    bd_: b
                }
            }
        }
        // }
    };

    ( $varitype:ident, Vari ) => {
        // {
        #[derive(Debug)]
        struct $varitype {
            data: VariData,
            avi_: Rc<RefCell<Vari>>,
            bvi_: Rc<RefCell<Vari>>
        }

        impl $varitype {
            fn new(v: Real, avi: Rc<RefCell<Vari>>, bvi: Rc<RefCell<Vari>>) -> $varitype {
                $varitype {
                    data: VariData::new(v),
                    avi_: avi.clone(),
                    bvi_: bvi.clone()
                }
            }
        }
        // }
    };
}
