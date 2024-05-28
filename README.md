# reperm_gen
General permutation trace generator

# Groups and Group Actions
todo: make a better tutorial
Group tutorial: https://en.wikipedia.org/wiki/Group_(mathematics)

# Generator overview
`Generator<'a, T>` is a trait that describes taking a source vector `#start(): Vec<T>`, and then by adding functions of type `Fn(T) => T` to the generator, the possible yielded sequences from it. Formally, a trace is a sequence of accesses, ie $\{x_i\}_{i \in I}$, where $`I`$ is an infinite or finite indexing set. Given $`(x_1, x_2, x_3, ..., x_n)`$, and a given $`f_j \in F`$, a set of the possible functions given,  the generator computes $`(x_{n + 1}, x_{n + 2}, x_{n + 3}, ..., x_{n + n}) = (f_j(x_1), f_j(x_2), f_j(x_3), ..., f_j(x_n))`$, and thus the running trace is $`(x_1, x_2, x_3, ..., x_n, x_{n + 1}, x_{n + 2}, x_{n + 3}, ..., x_{n + n})`$.

# Cycle
In this implementation, implementing the symmetric group and actions on some `T` was done by combining the two, which could possibly making the decoupling hard. This was done for ease of implementation, and not the case of effieciency. For `Cycle` to respect the set, we need to store a computation of the cycle on the groundset, which does gives up a lot of space since there are a lot of cycles ($`n!`$), but makes it easier to compute. 
