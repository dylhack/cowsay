PROJECT_DIR=$PWD
cd $PROJECT_DIR/cowbot && cargo test
cd $PROJECT_DIR/cowparse && cargo test --all-features
