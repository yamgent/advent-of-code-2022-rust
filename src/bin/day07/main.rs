const ACTUAL_INPUT: &str = include_str!("./input.txt");

#[derive(Debug, Clone, Copy)]
struct FileId(usize);

#[derive(Debug, Clone, Copy)]
struct DirId(usize);

#[derive(Debug)]
struct File<'a> {
    name: &'a str,
    size: usize,
}

impl<'a> File<'a> {
    fn new(name: &'a str, size: usize) -> Self {
        Self { name, size }
    }
}

#[derive(Debug)]
struct Dir<'a> {
    name: &'a str,
    parent: DirId,
    files: Vec<FileId>,
    subdirs: Vec<DirId>,
}

#[derive(Debug)]
struct Filesystem<'a> {
    files: Vec<File<'a>>,
    dirs: Vec<Dir<'a>>,
}

impl<'a> Filesystem<'a> {
    fn new() -> Self {
        Self {
            files: vec![],
            dirs: vec![Dir::new("", DirId(0))],
        }
    }

    fn add_file(&mut self, parent: DirId, file: File<'a>) -> FileId {
        self.files.push(file);

        let id = FileId(self.files.len() - 1);
        self.dirs.iter_mut().nth(parent.0).unwrap().files.push(id);
        id
    }

    fn add_dir(&mut self, parent: DirId, dir: Dir<'a>) -> DirId {
        self.dirs.push(dir);

        let id = DirId(self.dirs.len() - 1);
        self.dirs.iter_mut().nth(parent.0).unwrap().subdirs.push(id);
        id
    }

    fn root(&self) -> DirId {
        DirId(0)
    }

    fn parent(&self, current: DirId) -> DirId {
        self.dirs.iter().nth(current.0).unwrap().parent
    }

    fn child(&self, current: DirId, name: &str) -> DirId {
        *self
            .dirs
            .iter()
            .nth(current.0)
            .unwrap()
            .subdirs
            .iter()
            .find(|dirid| self.dirs.iter().nth(dirid.0).unwrap().name == name)
            .unwrap()
    }

    fn get_dir_filesizes(&self) -> Vec<usize> {
        let mut result = std::iter::repeat(None)
            .take(self.dirs.len())
            .collect::<Vec<_>>();

        fn traverse(fs: &Filesystem, result: &mut Vec<Option<usize>>, dir_id: DirId) -> usize {
            match result[dir_id.0] {
                Some(size) => size,
                None => {
                    let subdirs_size = fs
                        .dirs
                        .iter()
                        .nth(dir_id.0)
                        .unwrap()
                        .subdirs
                        .iter()
                        .map(|dir_id| traverse(fs, result, *dir_id))
                        .sum::<usize>();
                    let files_size = fs
                        .dirs
                        .iter()
                        .nth(dir_id.0)
                        .unwrap()
                        .files
                        .iter()
                        .map(|file_id| fs.files.iter().nth(file_id.0).unwrap().size)
                        .sum::<usize>();
                    result[dir_id.0] = Some(subdirs_size + files_size);
                    subdirs_size + files_size
                }
            }
        }

        traverse(&self, &mut result, self.root());
        result.iter().map(|v| v.unwrap()).collect()
    }
}

impl<'a> Dir<'a> {
    fn new(name: &'a str, parent: DirId) -> Self {
        Self {
            name,
            parent,
            files: vec![],
            subdirs: vec![],
        }
    }
}

fn parse_input(input: &str) -> Filesystem {
    let mut fs = Filesystem::new();

    let mut current = fs.root();

    input
        .trim()
        .lines()
        .fold(vec![], |mut acc, line| {
            if line.starts_with("$") {
                acc.push(vec![line]);
            } else {
                acc.iter_mut().last().unwrap().push(line);
            }
            acc
        })
        .into_iter()
        .for_each(|content| match content[0] {
            "$ cd /" => {
                current = fs.root();
            }
            "$ cd .." => {
                current = fs.parent(current);
            }
            "$ ls" => {
                content.iter().skip(1).for_each(|line| {
                    if line.starts_with("dir") {
                        let dirname = &line[4..];
                        fs.add_dir(current, Dir::new(dirname, current));
                    } else {
                        let (size, name) = line.split_once(' ').unwrap();
                        fs.add_file(current, File::new(name, size.parse::<usize>().unwrap()));
                    }
                });
            }
            _ => {
                if content[0].starts_with("$ cd") {
                    let dirname = &content[0][5..];
                    current = fs.child(current, dirname);
                } else {
                    panic!("Unknown instruction {}", content[0]);
                }
            }
        });
    fs
}

fn p1(input: &str) -> String {
    let fs = parse_input(input);

    fs.get_dir_filesizes()
        .iter()
        .filter(|v| **v <= 100000)
        .sum::<usize>()
        .to_string()
}

fn p2(input: &str) -> String {
    let _input = input.trim();
    "".to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("./sample.txt");

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "95437");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1315285");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
