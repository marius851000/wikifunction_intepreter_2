# Rust Wikifunction interpreter 2

The goal of this project is to be able to run code from Wikifunction in Rust. This is my second take on this idea, my first implementation being quite experimental and had its flow.

This version use what I think is called... forward evaluation? Basically, it recursively evaluate the data into it have fully evaluated what is requested.

## Implementation state:

- [x] parse XML dump
  - [ ] pre-process data such as, for example, Boolean are already stored as boolan and do not need conversion in the global context
- [ ] correctly handle case where the identity has been dereferenced (recursive identity reference resolution)
- [ ] running built-in function (e.g. Z844/boolean equality and Z802/if)
  - [ ] make sure to check input/output type are correct.
- [ ] running non-built-in function (e.g. Z10237/boolean inequality)
- [ ] running some tests (e.g. Z10238, a test for Z10237)
- [ ] proper handling of non-built-in type, including equality (making use of identity)
- [ ] proper handling of typed list
- [ ] proper handling of typed pair
- [ ] can evaluate all tests without crashes (not necessarelly without error)
- longer term stuff
  - [ ] run python/javascript implementation
  - [ ] fetch Wikidata element
  - [ ] multithreaded evaluation, as one process run multiple different request at the same time, and/or some version of (automatic) map/reduce
  - [ ] directly load element from wikifunctions rather than from the dump (with cache) (keep the option to load the dump available)
    - [ ] implement the orchestrator API?

## Architecture

for now, the bulk of the architecture I planned for is implemented (even before I ran my first implementation)

Every ZObject is represented by an implementation of WfDataType, and every implementer of WfDataType can be stored in one value of the WfData enum. The goal is for WfData to stay small, currently standing at 24 bytes. That way, when some processing needs to be done, it can be acquired quickly from the global data store, and object can be constructed quickly from another by cloning them (data too large for that size is stored behing Rc. Cloning is Rc is cheap. Between 4 to 8 times faster than performing an allocation).

(that’s quite a big difference from my previous implementation that relied more on moving data around. I still kept the idea of returning the WfData on error, in case some further processing is needed afterwhat, as that avoid having to clone the data before if it is needed afterward).

Unlike what I understands from the official orchestrator, this implementation rely quite heavily on specialiased types, not necessarelly because I need many specialiased operation, but because it should help with performance, and I find that more comfartable to works with. Plus this specialisation can be converted into the key/value of a ZObject. Those specialisation is a low level stuff, and should have no impact on evaluated code perceive their environment. It should be able to act as the official wikifunctions evaluator, modulo some bugs in their implementation.

Oh! And also, my interpreter makes sure function are only passed valid object. ZObjects that may hold invalid data is stored as-such, and will be converted to a more specialised representation (including a generic one that checks it against the keys of the type) before being returned, erroring if they are invalid (until eventually the next try/catch)
