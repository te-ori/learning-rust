struct ShirtColors {
    main: u8,
    second: u8
}

struct FootballTeam {
    name: String,
    establish: u16,
    shirt_colors: ShirtColors
       
}

struct TennisTeam {
    name: String,
    shirt_colors: ShirtColors
}

fn main() {
    let mut f_team = FootballTeam {
        name: "Fener".to_string(),
        establish: 1907,
        shirt_colors: ShirtColors {
            main: 1,
            second: 2,
        }
    };
    
    let t_team = TennisTeam {
        name: "Fener".to_string(),
        shirt_colors: ShirtColors {
            main: 1,
            second: 2,
        }
    };

    let write_establish = | establish: u16 | println!("establish: {}", establish);
    let write_establish_ref = | establish: &u16 | println!("establish: {}", establish);
    let write_establish_mut_ref = | establish: &mut u16 | {
          
        println!("establish: {}", establish);
        *establish += 100;
    };

    write_establish(f_team.establish);
    write_establish_mut_ref(&mut f_team.establish);
    write_establish_ref(&f_team.establish);
}
