# Review: your ALX bugs, and a Rust pull request

## Part 1: five bugs from your own C code

These are real bugs from your `alx-low_level_programming` repo. For each one, answer three questions:

1. What actually goes wrong when it runs?
2. If you wrote the same thing in Rust, would Rust stop it **at compile time**, stop it **at run time** (a panic with a clear message), or **not at all**?
3. Why?

### A. `0x0B-malloc_free/1-strdup.c`

```c
s = malloc((l + 1) * sizeof(char));
while (str[i] != '\0')
{
    s[i] = str[i];
    i++;
}
return (s);
```

### B. `0x0B-malloc_free/4-free_grid.c`

```c
while (height >= 0)
{
    free(grid[height]);
    height--;
}
```

### C. `0x0C-more_malloc_free/100-realloc.c` (when shrinking)

```c
new_ptr = malloc(new_size);
for (i = 0; i < old_size; i++)
    new_ptr[i] = temp_ptr[i];
```

### D. `0x13-more_singly_linked_lists/10-delete_nodeint.c` (deleting at index == length)

```c
if ((index > list_len) || (list_len == 0))
    return (-1);
/* ... walks to the end, then: */
temp = *head;          /* NULL */
current->next = temp->next;
```

### E. `0x13-more_singly_linked_lists/1-listint_len.c`

```c
while (h->next != NULL)
{
    if (h->n != '\0')
        elements++;
    h = h->next;
}
```

## Part 2: a Rust pull request

A teammate opens this PR: *"Add a helper to print badges for a station."* Write the review comments you'd leave: what's wrong, why, and what to do instead. It compiles and works. That's not the same as being right.

```rust
pub fn station_badges(names: Vec<String>, station: &String) -> Vec<String> {
    let mut badges = Vec::new();
    for name in names.clone() {
        let n = name.clone();
        badges.push(format!("{} - {}", n.to_uppercase(), station.clone()));
    }
    badges
}

// Called like this:
// let badges = station_badges(staff.clone(), &"NBO".to_string());
```
