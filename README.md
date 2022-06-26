
# nox

game submission for Mini Jam 109.

## RUNNING FOR DEVELOPMENT

install git pre commit hook
```
$ ./tools/devsetup.sh
```

run for development
```
$ cargo run
```

## TODO

- [x] enemy attacking
- [ ] rewrite movement to use physics
- [ ] attack collision detection
- [ ] automatic bullet despawning

## LIBRARIES

possible functionaity that can be abstracted into it's own library
- wave system
- utils (displacement component, health component)
- collision handling system (onCollide handler support)
