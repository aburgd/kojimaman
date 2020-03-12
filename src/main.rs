use colored::*;
use std::collections::HashMap;
use std::fmt;
use text_io::read;
use tiny_die::Die;

// COLLECTIONS SHORTHAND
type Information = HashMap<String, String>;
type Conditions = Vec<Condition>;
type Names = HashMap<Category, String>;

pub enum Condition {
    None,
    Man,
    Big,
    Old,
    Condition,
    Clone,
    Kojima,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Category {
    Normal,
    Occupational,
    Horny,
    The,
    Cool,
    Violent,
    LacksSubtext,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Category::Normal => write!(f, "Normal"),
            Category::Occupational => write!(f, "Occupational"),
            Category::Horny => write!(f, "Horny"),
            Category::The => write!(f, "The"),
            Category::Cool => write!(f, "Cool"),
            Category::Violent => write!(f, "Violent"),
            Category::LacksSubtext => write!(f, "Lacks Subtext"),
        }
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Condition::Man => write!(f, "Man"),
            Condition::Big => write!(f, "Big"),
            Condition::Old => write!(f, "Old"),
            Condition::Condition => write!(f, "Condition"),
            Condition::Clone => write!(f, "Clone"),
            Condition::Kojima => write!(f, "Kojima"),
            Condition::None => write!(f, "None"),
        }
    }
}

fn wait() {
    let drop: String = read!("{}\r\n");
    if drop.contains("\n") {
        std::process::exit(0);
    }
}

fn main() {
    // BEGINNING SECTIONS
    let how_many: u8 = section_how_many_names();
    let true_category: Category = section_name_category();
    let conditions: Conditions = section_determine_name_conditions();
    // COLLECTION SECTIONS
    let personal: Information = section_personal_information();
    let kojima: Information = section_kojima_information();
    // GENERATION SECTIONS
    // Find true_name
    let true_name = find_true_name(&true_category, &personal, &kojima, &conditions);
    // If 1 + 6 alternate, must have a true name.
    // all_names should contain true_name, but only do it if required
    if how_many == 1 {
        println!("{}: {}", true_category, true_name);
    } else {
        let mut all_names: Names = find_all_names(&personal, &kojima, &conditions);
        println!("{}", "True Name\n=========".bold());
        println!("{}: {}\n", true_category, true_name);
        all_names.remove(&true_category);
        println!("\n{}", "Alternates\n==========".bold());
        for name in all_names {
            println!("{}: {}", name.0, name.1);
        }
    }
    wait();
}

fn find_all_names(personal: &Information, kojima: &Information, conditions: &Conditions) -> Names {
    let categories: Vec<Category> = vec![
        Category::Normal,
        Category::Occupational,
        Category::Horny,
        Category::The,
        Category::Cool,
        Category::Violent,
        Category::LacksSubtext,
    ];
    let mut all_names: HashMap<Category, String> = HashMap::new();
    for category in categories {
        match category {
            Category::Normal => all_names.insert(category, section_normal(personal, conditions)),
            Category::Occupational => {
                all_names.insert(category, section_occupational(personal, kojima, conditions))
            }
            Category::Horny => all_names.insert(category, section_horny(personal, conditions)),
            Category::The => all_names.insert(category, section_the(personal, kojima, conditions)),
            Category::Cool => {
                all_names.insert(category, section_cool(personal, kojima, conditions))
            }
            Category::Violent => {
                all_names.insert(category, section_violent(personal, kojima, conditions))
            }
            Category::LacksSubtext => {
                all_names.insert(category, section_subtext(personal, conditions))
            }
        };
    }
    return all_names;
}

fn add_conditions(full_name: &String, conditions: &Conditions, personal: &Information) -> String {
    let full_name: Vec<&str> = full_name.split_whitespace().collect::<Vec<&str>>();
    let first_name = full_name[0];
    let last_name = full_name[full_name.len() - 1];
    // let mut condition_names: Vec<String> = Vec::new();
    let mut new_name: String = format!("{} {}", first_name, last_name);
    for condition in conditions {
        match condition {
            // because we don't have a way to handle the Clone condition (yet), we treat it as a None
            Condition::None | Condition::Clone => {
                new_name = format!("{} {}", first_name, last_name)
            }
            Condition::Big => new_name = format!("Big {}", new_name),
            Condition::Condition => {
                new_name = format!("{} {}", personal["body_condition"], new_name)
            }
            Condition::Kojima => new_name = format!("Hideo Kojima"),
            Condition::Man => new_name = format!("{}man", new_name),
            Condition::Old => new_name = format!("Old {}", new_name),
        }
    }
    return new_name;
}

fn find_true_name(
    name_category: &Category,
    personal: &Information,
    kojima: &Information,
    conditions: &Conditions,
) -> String {
    match name_category {
        Category::Normal => section_normal(&personal, &conditions),
        Category::Occupational => section_occupational(&personal, &kojima, &conditions),
        Category::Horny => section_horny(&personal, &conditions),
        Category::The => section_the(&personal, &kojima, &conditions),
        Category::Cool => section_cool(&personal, &kojima, &conditions),
        Category::Violent => section_violent(&personal, &kojima, &conditions),
        Category::LacksSubtext => section_subtext(&personal, &conditions),
    }
}

// Section 1
fn section_how_many_names() -> u8 {
    match Die::default().roll() {
        1..=5 => 1,
        6 => 7,
        _ => unreachable!(),
    }
}

// Section 2
fn section_personal_information() -> Information {
    let mut personal_information: Information = HashMap::new();
    println!("{} ", "What is your full name?".bold());
    personal_information.insert("full_name".to_string(), read!("{}\r\n"));
    println!(
        "{} ",
        "What is your occupation? Summarize in one -er verb.".bold()
    );
    personal_information.insert("one_word_occupation".to_string(), read!("{}\r\n"));
    println!(
        "{} ",
        "What was your first pet’s specific species/breed? If you've never had a pet, please answer with an animal you wish you owned.".bold()
    );
    personal_information.insert("first_pet_breed".to_string(), read!("{}\r\n"));
    println!(
        "{} ",
        "What’s your most embarrassing childhood memory? Summarise it with 2 words.".bold()
    );
    personal_information.insert("two_word_memory".to_string(), read!("{}\r\n"));
    println!(
        "{} ",
        "What is the object you'd least like to be stabbed by?".bold()
    );
    personal_information.insert("least_like_stabbed".to_string(), read!("{}\r\n"));
    println!(
        "{} ",
        "What is something you're good at? Summarize in one -ing verb.".bold()
    );
    personal_information.insert("good_at".to_string(), read!("{}\r\n"));
    println!(
        "{} ",
        "How many carrots do you believe you could eat in one sitting, if someone, like, forced you to eat as many carrots as possible?".bold()
    );
    personal_information.insert("carrots_one_sitting".to_string(), read!("{}\r\n"));
    println!("{} ", "What is your greatest intangible fear?".bold());
    personal_information.insert("intangible_fear".to_string(), read!("{}\r\n"));
    println!("{} ", "What is your greatest tangible fear?".bold());
    personal_information.insert("tangible_fear".to_string(), read!("{}\r\n"));
    println!(
        "{} ",
        "What is the last thing you did before starting this worksheet?".bold()
    );
    personal_information.insert("last_thing".to_string(), read!("{}\r\n"));
    println!(
        "{} ",
        "In one word, what condition is your body currently in?".bold()
    );
    personal_information.insert("body_condition".to_string(), read!("{}\r\n"));
    println!("{} ", "What is your favorite state of matter?".bold());
    personal_information.insert("favorite_state_matter".to_string(), read!("{}\r\n"));
    println!(
        "{} ",
        "What is a word your name sort of sounds like?".bold()
    );
    personal_information.insert("name_sounds_like".to_string(), read!("{}\r\n"));
    println!("{} ", "What is your Zodiac sign?".bold());
    personal_information.insert("zodiac".to_string(), read!("{}\r\n"));
    println!(
        "{} ",
        "If you could define your personality in one word, what would it be?".bold()
    );
    personal_information.insert("one_word_personality".to_string(), read!("{}\r\n"));
    return personal_information;
}

// Section 3
fn section_kojima_information() -> Information {
    let mut kojima_information: Information = HashMap::new();
    println!(
        "{} ",
        "Who is your favorite film character? Must be played by Kurt Russell.".bold()
    );
    kojima_information.insert("favorite_kurt_russell".to_string(), read!("{}\r\n"));
    println!(
        "{} ",
        "What is the last word of the title of your favorite Kubrick film?".bold()
    );
    kojima_information.insert("favorite_kubrick_film".to_string(), read!("{}\r\n"));
    println!(
        "{} ",
        "What is the first word in the title of your favorite Joy Division album?".bold()
    );
    kojima_information.insert("favorite_joy_division".to_string(), read!("{}\r\n"));
    println!(
        "{} ",
        "What is a scientific term you picked up recently?".bold()
    );
    kojima_information.insert("recent_scientfic_term".to_string(), read!("{}\r\n"));
    println!(
        "{} ",
        "What is a piece of military hardware you think looks cool, even though war is bad?".bold()
    );
    kojima_information.insert("cool_military_war_bad".to_string(), read!("{}\r\n"));
    println!(
        "{} ",
        "What is something you'd enjoy watching Mads Mikkelsen do? Summarize in one word.".bold()
    );
    kojima_information.insert("mads_mikkelsen_amazing".to_string(), read!("{}\r\n"));
    return kojima_information;
}

// Section 4
fn section_determine_name_conditions() -> Conditions {
    let dee_four: Die = Die::new(4);
    let man_condition: Condition = match dee_four.roll() {
        4 => Condition::Man,
        1..=3 | _ => Condition::None,
    };
    let dee_eight: Die = Die::new(8);
    let condition_condition: Condition = match dee_eight.roll() {
        6 => Condition::Big,
        7 => Condition::Old,
        8 => Condition::Condition,
        1..=5 | _ => Condition::None,
    };
    let dee_twelve: Die = Die::new(12);
    let clone_condition: Condition = match dee_twelve.roll() {
        12 => Condition::Clone,
        1..=11 | _ => Condition::None,
    };
    let dee_hundred: Die = Die::new(100);
    let kojima_condition: Condition = match dee_hundred.roll() {
        69 => Condition::Kojima,
        1..=68 | 70..=100 | _ => Condition::None,
    };
    vec![
        man_condition,
        condition_condition,
        clone_condition,
        kojima_condition,
    ]
}

// Section 5
fn section_name_category() -> Category {
    let dee_twenty = Die::new(20);
    match dee_twenty.roll() {
        1 => Category::Normal,
        2..=6 => Category::Occupational,
        7..=10 => Category::Horny,
        11..=13 => Category::The,
        14..=17 => Category::Cool,
        18..=19 => Category::Violent,
        20 => Category::LacksSubtext,
        _ => unreachable!(),
    }
}

// Section 6
fn section_normal(personal: &Information, conditions: &Conditions) -> String {
    // From Section 2, Number 1 (2.1)
    let full_name = &personal["full_name"];
    add_conditions(&full_name, conditions, personal)
}

// Section 7
fn section_occupational(
    personal: &Information,
    kojima: &Information,
    conditions: &Conditions,
) -> String {
    // 1 - 2.15, one_word_personality
    // 2 - 2.6, good_at
    // 3 - 2.13, name_sounds_like
    // 4 - 3.16, favorite_kurt_russell
    let last_name = &personal["one_word_occupation"];
    let dee_four = Die::new(4);
    let first_name = match dee_four.roll() {
        1 => &personal["one_word_personality"],
        2 => &personal["good_at"],
        3 => &personal["name_sounds_like"],
        4 => &kojima["favorite_kurt_russell"],
        _ => unreachable!(),
    };
    let full_name = format!("{} {}", first_name, last_name);
    add_conditions(&full_name, conditions, personal)
}

// Section 8
fn section_horny(personal: &Information, conditions: &Conditions) -> String {
    // 2.3, first_pet_breed
    // 1 - 2.12, favorite_state_matter
    // 2 - Naked
    // 3 - 2.6, good_at
    // 4 - 2.14, zodiac
    let last_name = &personal["first_pet_breed"];
    let dee_four: Die = Die::new(4);
    let first_name = match dee_four.roll() {
        1 => &personal["favorite_state_matter"],
        2 => "Naked",
        3 => &personal["good_at"],
        4 => &personal["zodiac"],
        _ => unreachable!(),
    };
    let full_name: String = format!("{} {}", first_name, last_name);
    add_conditions(&full_name, conditions, personal)
}

// Section 9
fn section_the(personal: &Information, kojima: &Information, conditions: &Conditions) -> String {
    let first_name: String = "The".to_string();
    let dee_four: Die = Die::new(4);
    let last_name = match dee_four.roll() {
        // 1 - 2.8 intangible_fear
        // 2 - 2.9 tangible_fear
        // 3 - 2.4a two_word_memory
        // 4 - 3.20 cool_military_war_bad
        1 => &personal["intangible_fear"],
        2 => &personal["tangible_fear"],
        3 => &personal["two_word_memory"],
        4 => &kojima["cool_military_war_bad"],
        _ => unreachable!(),
    };
    let full_name: String = format!("{} {}", first_name, last_name);
    add_conditions(&full_name, conditions, personal)
}

// Section 10
fn section_cool(personal: &Information, kojima: &Information, conditions: &Conditions) -> String {
    // 3.21a
    let first_name = &kojima["mads_mikkelsen_amazing"];
    let dee_six: Die = Die::default();
    // 1 - 3.17, favorite_kubrick_film
    // 2 - 3.18, favorite_joy_division
    // 3 - 3.19, recent_scientific_term
    // 4 - 2.6, good_at
    // 5 - 2.8, intangible_fear
    // 6 - 2.13, name_sounds_like
    let last_name = match dee_six.roll() {
        1 => &kojima["favorite_kubrick_film"],
        2 => &kojima["favorite_joy_division"],
        3 => &kojima["recent_scientific_term"],
        4 => &personal["good_at"],
        5 => &personal["intangible_fear"],
        6 => &personal["name_sounds_like"],
        _ => unreachable!(),
    };
    let full_name: String = format!("{} {}", first_name, last_name);
    add_conditions(&full_name, conditions, personal)
}

// Section 11
fn section_violent(
    personal: &Information,
    kojima: &Information,
    conditions: &Conditions,
) -> String {
    // 2.5, least_like_stabbed
    let first_name = &personal["least_like_stabbed"];
    let dee_four: Die = Die::new(4);
    // 1 - 3.19
    // 2 - 2.12
    // 3 - 3.20
    // 4 - 2.9
    let last_name = match dee_four.roll() {
        1 => &kojima["recent_scientific_term"],
        2 => &personal["favorite_state_matter"],
        3 => &kojima["cool_military_war_bad"],
        4 => &personal["tangible_fear"],
        _ => unreachable!(),
    };
    let full_name: String = format!("{} {}", first_name, last_name);
    add_conditions(&full_name, conditions, personal)
}

// Section 12
fn section_subtext(personal: &Information, conditions: &Conditions) -> String {
    let full_name: String = format!("{}", &personal["last_thing"]);
    add_conditions(&full_name, &conditions, &personal)
}
