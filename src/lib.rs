pub mod poly {
    pub mod polynomial;
    pub mod monomial;
    pub mod univariate_polynomial;
}
pub mod numbers {
    pub mod sets {
        pub mod Class;
        
    }
    pub mod numbers;
    pub mod classes {
        pub mod QQ;
        pub mod RR;
        pub mod ZZ;
    }

    pub mod instances {
        pub mod QQ_instance;
        pub mod RR_instance;
        pub mod ZZ_instance;
    }
    
}
pub mod variables {
    pub mod vars;
}

pub mod utilities {
    pub mod utils;
}

pub mod test {
    pub mod test_ZZ;
    pub mod test_RR;
    pub mod test_QQ;
}

pub mod algebras {

    pub mod Rings {
        pub mod classes {
            pub mod PolynomialRing;
        }

        pub mod instances {
            pub mod PolynomialRing_instance;
        }
    }


    pub mod FiniteField {
        pub mod classes {
            pub mod Zmod;
        }

        pub mod instances {
            pub mod Zmod_instance;
        }
    }
}

pub mod generators {
    pub mod random;
}

// use num_bigint;
// use num_bigint;
// use bigdecimal;