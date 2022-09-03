use git::Git;

mod git;

fn main() {

    let g : Git = Git{};
    g.untracked_file();

}
