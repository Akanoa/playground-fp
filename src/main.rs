pub trait Map<Res> {
    /// Map a function which takes parameter by move or copy
    fn map<F: FnOnce(Self) -> Res>(self, f: F) -> Res
    where
        Self: Sized,
    {
        f(self)
    }

    /// Map a function which takes parameter by reference.
    fn map_ref<F: FnOnce(&Self) -> Res>(&self, f: F) -> Res {
        f(self)
    }

    /// Map a function which takes parameter by mutable reference.
    fn map_mut<F: FnOnce(&mut Self) -> Res>(&mut self, f: F) -> Res {
        f(self)
    }
}

pub struct Expenses;

impl Expenses {
    pub fn get_transport_expenses(turnover: f32) -> f32 {
        turnover - 2400.0
    }

    pub fn get_operating_expenses(turnover: f32) -> f32 {
        turnover - 15000.0
    }

    pub fn get_deductible_taxes(turnover: f32) -> f32 {
        turnover - 3000.0
    }

    pub fn get_remuneration(turnover: f32) -> f32 {
        turnover - 45000.0
    }

    pub fn get_exceptional_expenses(turnover: f32) -> f32 {
        turnover - 2000.0
    }

    pub fn transform_str1(element: &str) -> String {
        format!("{element} toto")
    }

    pub fn transform_str2(element: &str) -> String {
        format!("{element} tata")
    }
}

impl<T: ?Sized, Res> Map<Res> for T {
    // use default definitions...
}

#[derive(Debug)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub civility: String,
}

fn main() {

    let turnover: f32 = 100000.0;

    let profit = turnover
    .map(Expenses::get_remuneration)
    .map(Expenses::get_deductible_taxes)
    .map(Expenses::get_operating_expenses)
    .map(Expenses::get_transport_expenses)
    .map(Expenses::get_exceptional_expenses);

    dbg!(profit);

    let lazy_profit = turnover.map(|data| {
        move || {
            data.map(Expenses::get_remuneration)
            .map(Expenses::get_deductible_taxes)
            .map(Expenses::get_operating_expenses)
            .map(Expenses::get_transport_expenses)
            .map(Expenses::get_exceptional_expenses)
        }
    });

    dbg!(execute_lazy(lazy_profit));

    let person = Person {
        first_name: "Coucou Mazlum".into(),
        last_name: "Toto".into(),
        civility: "Madrid".into(),
    };
    
    let result_str = person
    .map_ref(|p| Expenses::transform_str1(&p.first_name))
    .map_ref(|s| Expenses::transform_str2(&s));

dbg!(result_str);
dbg!(person);



}

fn execute_lazy<T, F: Fn() -> T>(f: F) -> T {
    f()
}
