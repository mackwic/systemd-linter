# This script takes care of testing your crate

set -ex

# TODO This is the "test phase", tweak it as you see fit
main() {
    cross build --target $TARGET
    cross build --target $TARGET --release

    if [[ $DISABLE_TESTS = "1" ]]; then
        return
    fi

    for subcrate in crates/*
    do
        cd $subcrate
        cross test --target $TARGET
        cross test --target $TARGET --release
        cd -
    done

    cross test --target $TARGET
    cross test --target $TARGET --release

    cross run --target $TARGET -- --help
    cross run --target $TARGET --release -- --help
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
