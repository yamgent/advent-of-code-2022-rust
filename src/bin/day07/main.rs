const ACTUAL_INPUT: &str = include_str!("./input.txt");

#[derive(Debug, Clone, Copy)]
struct FileId(usize);

#[derive(Debug, Clone, Copy)]
struct DirId(usize);

#[derive(Debug)]
struct File<'a> {
    // we never really use name to compute our answer, but it is useful for debugging
    #[allow(dead_code)]
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

#[derive(Debug)]
struct Filesystem<'a> {
    files: Vec<File<'a>>,
    dirs: Vec<Dir<'a>>,
}

impl<'a> Filesystem<'a> {
    fn from_input(input: &'a str) -> Self {
        let mut fs = Filesystem {
            files: vec![],
            dirs: vec![Dir::new("", DirId(0))],
        };

        let mut current = fs.root();

        input
            .trim()
            .lines()
            .fold(vec![], |mut acc, line| {
                if line.starts_with('$') {
                    acc.push(vec![line]);
                } else {
                    acc.iter_mut().last().unwrap().push(line);
                }
                acc
            })
            .into_iter()
            .for_each(|section| {
                let (command, body) = section.split_at(1);
                let command = command[0];
                match command {
                    "$ cd /" => {
                        current = fs.root();
                    }
                    "$ cd .." => {
                        current = fs.parent(current);
                    }
                    "$ ls" => {
                        body.iter().for_each(|line| {
                            if line.starts_with("dir") {
                                let dirname = &line[4..];
                                fs.add_dir(current, Dir::new(dirname, current));
                            } else {
                                let (size, name) = line.split_once(' ').unwrap();
                                fs.add_file(
                                    current,
                                    File::new(name, size.parse::<usize>().unwrap()),
                                );
                            }
                        });
                    }
                    _ => {
                        if command.starts_with("$ cd") {
                            let dirname = &command[5..];
                            current = fs.child_dir(current, dirname);
                        } else {
                            panic!("Unknown instruction {}", command);
                        }
                    }
                }
            });
        fs
    }

    fn add_file(&mut self, parent: DirId, file: File<'a>) -> FileId {
        self.files.push(file);

        let id = FileId(self.files.len() - 1);
        self.dirs[parent.0].files.push(id);
        id
    }

    fn add_dir(&mut self, parent: DirId, dir: Dir<'a>) -> DirId {
        self.dirs.push(dir);

        let id = DirId(self.dirs.len() - 1);
        self.dirs[parent.0].subdirs.push(id);
        id
    }

    fn root(&self) -> DirId {
        DirId(0)
    }

    fn parent(&self, current: DirId) -> DirId {
        self.dirs[current.0].parent
    }

    fn child_dir(&self, current: DirId, name: &str) -> DirId {
        *self.dirs[current.0]
            .subdirs
            .iter()
            .find(|dirid| self.dirs[dirid.0].name == name)
            .unwrap()
    }

    fn get_dirs_filesizes(&self) -> Vec<usize> {
        let mut result = std::iter::repeat(0usize)
            .take(self.dirs.len())
            .collect::<Vec<_>>();

        fn traverse(fs: &Filesystem, result: &mut Vec<usize>, dir_id: DirId) -> usize {
            let subdirs_size = fs.dirs[dir_id.0]
                .subdirs
                .iter()
                .map(|dir_id| traverse(fs, result, *dir_id))
                .sum::<usize>();
            let files_size = fs.dirs[dir_id.0]
                .files
                .iter()
                .map(|file_id| fs.files[file_id.0].size)
                .sum::<usize>();
            result[dir_id.0] = subdirs_size + files_size;
            subdirs_size + files_size
        }

        traverse(self, &mut result, self.root());
        result
    }
}

fn p1(input: &str) -> String {
    let fs = Filesystem::from_input(input);

    fs.get_dirs_filesizes()
        .into_iter()
        .filter(|v| *v <= 100_000)
        .sum::<usize>()
        .to_string()
}

fn p2(input: &str) -> String {
    let fs = Filesystem::from_input(input);
    let mut sizes = fs.get_dirs_filesizes();
    let used = sizes[0];
    sizes.sort();
    sizes
        .into_iter()
        .find(|s| used - s <= 40_000_000)
        .unwrap()
        .to_string()
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
        assert_eq!(p2(SAMPLE_INPUT), "24933642");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "9847279");
    }
}
