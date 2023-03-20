pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{:#?}\n", minefield);
    let data = minefield
        .iter()
        .enumerate()
        .map(|(line, x)| {
            x.as_bytes()
                .iter()
                .enumerate()
                .map(|(index, val)| {
                    if *val == '*' as u8 {
                        return String::from("*");
                    } else if *val == ' ' as u8 {
                        let mut count = 0;

                        if line > 0 {
                            if index > 0 && minefield[line - 1].as_bytes()[index - 1] == '*' as u8 {
                                count += 1;
                            }
                            if minefield[line - 1].as_bytes()[index] == '*' as u8 {
                                count += 1;
                            }
                            if index < x.len() - 2
                                && minefield[line - 1].as_bytes()[index + 1] == '*' as u8
                            {
                                count += 1;
                            }
                        }
                        if line < minefield.len() - 1 {
                            if index > 0 && minefield[line + 1].as_bytes()[index - 1] == '*' as u8 {
                                count += 1;
                            }
                            if minefield[line + 1].as_bytes()[index] == '*' as u8 {
                                count += 1;
                            }
                            if index < x.len() - 2
                                && minefield[line + 1].as_bytes()[index + 1] == '*' as u8
                            {
                                count += 1;
                            }
                        }
                        if index > 0 && minefield[line].as_bytes()[index - 1] == '*' as u8 {
                            count += 1;
                        }
                        if index < x.len() - 2 && minefield[line].as_bytes()[index + 1] == '*' as u8
                        {
                            count += 1;
                        }
                        if count == 0 {
                            return String::from(" ");
                        } else {
                            return count.to_string();
                        }
                    } else {
                        return val.to_string();
                    }
                })
                .collect::<String>()
            // println!("Item {} = {}", i, x);
        })
        .collect();

    data
}
