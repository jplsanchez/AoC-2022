use std::slice;

impl Data {
    pub fn decode(encoded: Vec<String>) -> Data {
        let mut iterator = encoded.iter();
        let stacks_quantity: usize = encoded[0].len() / 4 + 1;

        Data {
            stacks: Data::decode_stacks(&mut iterator, stacks_quantity),
            commands: Data::decode_commands(&mut iterator),
        }
    }

    fn decode_stacks(iterator: &mut slice::Iter<'_, String>, stacks_quantity: usize) -> Vec<Stack> {
        let mut stacks: Vec<Stack> = (0..stacks_quantity).map(|_| Vec::new()).collect();

        // walk through the lines of encoded data
        while let Some(line) = iterator.next() {
            match line.as_str() {
                "" => break,
                _ => {
                    let line = line.chars().collect::<Vec<char>>();
                    for i in (1..line.len()).step_by(4) {
                        // divide through the stacks "[Q] [Q] [B] [D] [J] [W] [H] [R] [J]"
                        let row = i / 4;
                        stacks[row].push(line[i])
                    }
                }
            }
        }

        stacks.iter_mut().for_each(|stack| {
            stack.pop(); // Removing id (id is pos+1 => for stack[2] then position is 3)
            stack.reverse(); // puting the stack in the right order => Last element is the top
            stack.retain(|&c| c != ' ') // removing black spaces
        });

        stacks
    }

    fn decode_commands(iterator: &mut slice::Iter<'_, String>) -> Vec<Command> {
        let mut commands: Vec<Command> = Vec::new();

        while let Some(line) = iterator.next() {
            commands.push(Command::from_line(line));
        }
        commands
    }
}

pub struct Data {
    pub stacks: Vec<Stack>,
    pub commands: Vec<Command>,
}

type Stack = Vec<char>;

pub struct Command {
    pub quantity: usize,
    pub source: usize,
    pub target: usize,
}

impl Command {
    fn from_line(line: &str) -> Command {
        let delimiters = &["move", "from", "to", " "];
        let regex = regex::Regex::new(delimiters.join("|").as_str()).unwrap();

        let data = &mut regex
            .split(line)
            .filter(|&v| !v.is_empty())
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        Command {
            quantity: data[0],
            source: data[1],
            target: data[2],
        }
    }
}
