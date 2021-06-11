# Undead
A full-nonsense (yes, it's a nonsense) [Zombie](https://www.dangermouse.net/esoteric/zombie.html) language __*dialect*__ interpreter written in Rust.

## Why?
I don't know. 

## How?
Read a bit lower in this file.

## Running it
This interpreter runs on a single command:
```cli
cargo run [filename]
```
Where `[filename]` is a valid .zombie file. 

# The language specifics
As this is a dialect, heavily inspired by the original language, there are a lot of similarities and key differences.  

## Keywords 
Undead uses the following keywords:  
`summon` - A keystone to define new entity.  
`bind` - Sets the entity and it's bounds.  
`task` - Marks a task to be done by the parent entity.  
`done` - Marks the end of the current task.  
`animate` - Executes all tasks in the given entity in a way, specific for the entity itself.  
`banish` - Banishes the entity from the codebase. Non-reversible.  
`use` -  Same as using `animate` followed by `banish` for the same entity.  
`read_about` - Displays a friendly piece information about the entity.  

## Differences from the original
The original Zombie language allows code like this:
```
HelloWorld is a zombie
summon
	task SayHello
		say "Hello World!"
	animate
animate
```

This code is not only _NOT ALLOWED_ but it will also produce an error.
The correct way of writing the same code in Undead is:
```
HelloWorld is a Zombie
summon
	task SayHello
	    say "Hello World! \n"
	done
bind
animate HelloWorld
```

### Inactive-first state
The main difference with Zombie is the fact that entities and 
tasks in Undead are **inactive** by default, and won't animate until 
the entity is explicitly told to do so. 

### Entity types capitalization
Entity types in undead __*MUST*__ be capitalized, with respect to the entity. 
Otherwise, the summoning will fail, thus no entity will be created.  

Example:
```
HelloWorld is a Zombie // non-capitalized entity type
summon
	task SayHello
	    say "Hello World! \n"
	done
bind
animate HelloWorld
```
This code snippet will produce the following output:
```
Error on line 7: No entity to with name "HelloWorld" was found.
```

### Run + banish
In Undead, you can run and directly banish an entity by using the `use` keyword. 
This assures that no entity will live more than it's needed to.  
Example:
```
HelloWorld is a Zombie
summon
	task SayHello
	    say "Hello World! \n"
	done
bind

use HelloWorld
```

Nothing changes visually, but if we decide to get the available 
information about it by using the `read_about` directive we'll 
get the following error:
```
Hello World! 
Error on line 9: No entity to with name "HelloWorld" was found.
```
Meaning that the entity does no longer exist.

### What are top-level commands?
Undead, contrasting to Zombie, makes difference between different 
commands upon their validation. This means that, using a command 
in the wrong place __will__ lead to an error.  
Example:
```Test is a Zombie
summon
bind

HelloWorld is a Zombie
summon
	task SayHello
	    say "Hello World! \n"
	    use test //this will produce an error
	done
bind
```
We will get the error `Error on line 9: Attempted top-level execution within entity/task bounds.`,
meaning that the `use` keyword must NOT be used within secondary-levels (entity or task scopes).

The top level commands in Undead are:
`use`, `animate` and `banish`. More might follow later on.