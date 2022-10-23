zang::zang!{
    struktura Nuqta {
        x: f64,
        y: f64,
    }
    
    amal Nuqta {
        funksiya yangi(x: f64, y: f64) -> Nuqta {
            Nuqta { x: x, y: y }
        }
    }
    
    struktura Tortburchak {
        p1: Nuqta,
        p2: Nuqta,
    }
    
    amal Tortburchak {
        // To'rtburchak yuzasi
        funksiya yuza(&men) -> f64 {
            let Nuqta { x: x1, y: y1 } = men.p1;
            let Nuqta { x: x2, y: y2 } = men.p2;
            ((x1 - x2) * (y1 - y2)).abs()
        }
        // To'rtburchak perimetri
        funksiya perimeter(&men) -> f64 {
            let Nuqta { x: x1, y: y1 } = men.p1;
            let Nuqta { x: x2, y: y2 } = men.p2;
    
            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }
    }

    funksiya asosiy() {
        let tortburchak = Tortburchak {
            p1: Nuqta::yangi(0.0, 0.0),
            p2: Nuqta::yangi(3.0, 4.0),
        };
    
        liniyachopetish!("To'rtburchak perimeteri: {}", tortburchak.perimeter());
        liniyachopetish!("To'rtburchak yuzasi: {}", tortburchak.yuza());
    }
}
