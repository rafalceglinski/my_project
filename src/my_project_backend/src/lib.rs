use std::cell::RefCell;

thread_local! {
    static MSG: RefCell<String> = RefCell::new(String::new());
} //RefCel i threadlocal dba o to abysmy nie usuneli tej zmiennej. Funkcja with zabezpiecza nas zebysmy nie usuneli danej zmiennej

#[ic_cdk::query]
fn get_msg() -> String {
    //let a = String::from("123");
    //let b = a.clone();
    //let b = &a; //ta referencja pokazuje ono na pole a, czyli gadanie na b jest taka sama jak na a 
    //let c = a;
    MSG.with(|msg| msg.borrow().clone())
}

#[ic_cdk::update]
fn set_msg(new_msg: String) {
    // COUNTER.replace(n);  // requires #![feature(local_key_cell_methods)]
    MSG.with(|msg| *msg.borrow_mut() = new_msg); //gwiazdka to kopi w ruscie, to kopia w ruscie czyli cos co czysci typy
    //new_msg: String
    //msg.borrow_mut() : &mut String
    //&mut String != String ???
    //*(&mut String ) == String czyli to robi gwiazdka
    //ksiazka o ruscie dock.rust-lang.org/book/ 
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
