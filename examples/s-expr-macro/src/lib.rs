#[macro_export]
macro_rules! s {
    (($($x: tt)+)) => {
        s!($($x)+)
    };
    (num $x: literal) => {
        $x
    };
    (add ($($x: tt)+) $(($($y: tt)+))+) => {
        s!($($x)+) $(+ s!($($y)+))+
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn adding_two() {
        let result = s!(add (num 10) (num 20));
        assert_eq!(result, 30);
    }

    #[test]
    fn adding_lots() {
        let result = s!(add (num 10) (num 20) (num 10) (add (num 5) (num 2)));
        assert_eq!(result, 47);
    }

    #[test]
    fn adding_lots_ws() {
        let result = s!(
            add
                (num 10)
                (num 20)
                (num 10)
                (add
                    (num 5)
                    (num 2)));
        assert_eq!(result, 47);
    }
}
