# Understanding bitwise

My personal experience of trying to understand bitwise by solving the tasks.

All tasks use unsigned 32 bit numbers for the sake of simplicity.

Note the [*expression precedence*](https://doc.rust-lang.org/reference/expressions.html#expression-precedence).

### Useful Commands

- Build:

  ```shell
  cargo build
  ```

- Run clippy (static analyzer):

  ```shell
  cargo clippy
  ```

- Run tests:

  ```shell
  cargo test
  ```

- Generate documentation:

  ```shell
  cargo doc
  ```

## Task 1

Find 2ⁿ (using bitwise operations).

### Solution Details

We're using *left shift* operation to achieve exponentiation of 2.  
Let's consider binary representation of some number.

```
… 0 1 0 0 0 1 1 = … + 0×2ⁿ + … + 0×2⁶ + 1×2⁵ + 0×2⁴ + 0×2³ + 0×2² + 1×2¹ + 1×2⁰ = 1×2⁵ + 1×2¹ + 1×2⁰ = 32 + 2 + 1 = 35
```

As we can see 1 at corresponding position in binary representation gives us corresponding power of 2.  
So the solution will be shifting 1 from *0* position to *n* position:

```
1 << n
```

We should also mind the overflow of integer, as it fixed size. So *n* should be in a valid range.

## Task 2

Get string representing the binary representation of a number (using bitwise operations).

### Solution Details

We're looping *right shift* operation over the number until it gets 0.  
Before shifting we're applying

```
number & 1
```

every iteration to get a bit from *0* position:

```
  1 0 0 0 1 1
&
  0 0 0 0 0 1
  -----------
  0 0 0 0 0 1

  1 0 0 0 1 1
>>
            1
  -----------
  0 1 0 0 0 1


  0 1 0 0 0 1
&
  0 0 0 0 0 1
  -----------
  0 0 0 0 0 1

  0 1 0 0 0 1
>>
            1
  -----------
  0 0 1 0 0 0

  0 0 1 0 0 0
&
  0 0 0 0 0 1
  -----------
  0 0 0 0 0 0
  
  …
  
  0 0 0 0 0 1
>>
            1
  -----------
  0 0 0 0 0 0


  END
```

## Task 3

Find ones count in binary representation of the number.

### Solution Details

#### Solution 1

We're using same method used in Task 2. When we're met bit that is 1 (while iterating) we're inc count variable.

#### Solution 2

We're looping *bitwise and* over the *number* and *number - 1* until the *number* gets 0.

```
  1 0 0 0 1 1
-
  0 0 0 0 0 1
  -----------
  1 0 0 0 1 0

  1 0 0 0 1 1
&
  1 0 0 0 1 0
  -----------
  1 0 0 0 1 0


  1 0 0 0 1 0
-
  0 0 0 0 0 1
  -----------
  1 0 0 0 0 1

  1 0 0 0 1 0
&
  1 0 0 0 0 1
  -----------
  1 0 0 0 0 0


  1 0 0 0 0 0
-
  0 0 0 0 0 1
  -----------
  0 1 1 1 1 1

  1 0 0 0 0 0
&
  0 1 1 1 1 1
  -----------
  0 0 0 0 0 0
```

Loop iterations count is the ones count.

The idea of the algorithm is that *number - 1* operation will take away exactly one the one from binary representation of the *number* at some position.  
However, it can mess the other positions (*0* may turn into *1*). *Bitwise and* with the number before it was subtracted solves this issue.

## Task 4

Find index of [*highest order bit*](https://commoncog.com/blog/highest-order-bit/) in binary representation of the number.

### Solution Details

#### Solution 1

We're using same method used in Task 2. We're inc index variable every iteration.

#### Solution 2

In this solution we're using a special *threshold* value. Initially it's a number with highest possible bit set to 1 and others bits set to 0.
We're iterating while *number* is less than that *threshold* and every iteration we're shifting *threshold* one position right. And dec index (initially it has the value of max position).

```
  1 0 0 0 1 1 < 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
                ▲

  1 0 0 0 1 1 < 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
                  ▲
                  
  1 0 0 0 1 1 < 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
                    ▲

  …

  1 0 0 0 1 1 < 
1 0 0 0 0 0 0
▲

  1 0 0 0 1 1 < 
  1 0 0 0 0 0
  ▲
  
  1 0 0 0 1 1 < 
  0 1 0 0 0 0

  END
```

#### Solution 3

It's similar to the solution 2. But instead of comparing "less than" we're comparing *number & threshold* to be equal *threshold*. Also we're computing *threshold* by raising two to the corresponding power each iteration instead of performing right shift over the *threshold*.

```
  …

  0 1 0 0 0 1 1
&
  1 0 0 0 0 0 0
  ▲
  -------------
  0 0 0 0 0 0 0
  
  0 == 1 0 0 0 1 1
  
  0 1 0 0 0 1 1
&
  0 1 0 0 0 0 0
    ▲
  -------------
  0 1 0 0 0 0 0
  
  0 1 0 0 0 0 0 == 0 1 0 0 0 0 0
  
  END
```

## Task 5

Set the specific bit of the number to 1.

### Solution Details

Let's have the number with specific bit set to 1 and others bits set to 0.  
Then we're just performing *bitwise or* over this number and original number.

```
  index_of_specific_bit = 3
  
  1 << 3 = 1 0 0 0

  1 0 0 0 1 1
|
  0 0 1 0 0 0
  -----------
  1 0 1 0 1 1
```

## Task 6

Set the specific bit of the number to 0.

### Solution Details

#### Solution 1

As follows:

```
(number | 1 << index) - (1 << index)
```

First, we're setting the bit and then we just subtracting corresponding bit. If the bit value was initially 1 it will remain 1 after *bitwise or* and if it wasn't then it will become 1.

```
  index_of_specific_bit = 1

  1 0 0 0 1 1
|
  0 0 0 0 1 0
  -----------
  1 0 0 0 1 1
  
  1 0 0 0 1 1
-
  0 0 0 0 1 0
  -----------
  1 0 0 0 0 1
```

#### Solution 2

As follows:

```
number & (number ^ 1 << index)
```

Xor operation have the following property:

```
1 ^ 1 == 0
0 ^ 1 == 1
```

As you can see, it inverts the given value in case of xoring 1.  
Also note that:

```
1 ^ 0 == 1
0 ^ 0 == 0
```
Xoring 0 doesn't change the original value.  
Using these we're inverting corresponding bit and then performing *bitwise and* with the original number. Because of the bit is inverted, the *bitwise and* always gives *0* at that position.

```
  index_of_specific_bit = 1

  1 0 0 0 1 1
^
  0 0 0 0 1 0
  -----------
  1 0 0 0 0 1
  
  1 0 0 0 1 1
&
  1 0 0 0 0 1
  -----------
  1 0 0 0 0 1
```

#### Solution 3

As follows:

```
number & ! (1 << index)
```

There's another way to invert the given bit. It's performing *bitwise not* over the number with specific bit set to 1 and others bits set to 0. Because of the bit is inverted, the *bitwise and* always gives *0* at that position. And all other bits are remaining untouched, because they were *bitwise and*ed over the 1.

```
  index_of_specific_bit = 1

! 0 0 0 0 1 0
  -----------
  1 1 1 1 0 1
  
  1 0 0 0 1 1
&
  1 1 1 1 0 1
  -----------
  1 0 0 0 0 1
```

## Task 7

Invert the specific bit of the number to 0.

### Solution Details

As follows:

```
number ^ 1 << index
```

Inverting bit procedure was described in the Solution 2 of the Task 6.

## Task 8

Perform left circular shift by certain number of positions for the given number.

### Solution Details

As follows:

```
number << count | number >> TYPE_BITS_COUNT - count
```

*TYPE_BITS_COUNT* - bits count of the variable type.  
*count* - certain number of positions to be shifted by.

The idea of the algorithm is that we're just positioning the bits in the way to correspond the left circular shift operation. We use two units for that. One's achieved by *bitwise shift left* and another by *bitwise shift right*. Then we're uniting two parts by *bitwise or*. Voila!

```
  TYPE_BITS_COUNT = 8
  count = 2

  1 0 0 0 0 0 1 1
<<
                2
  ---------------
  0 0 0 0 1 1 0 0
  
  1 0 0 0 0 0 1 1
>>
                6
  ---------------
  0 0 0 0 0 0 1 0
  
  0 0 0 0 1 1 0 0
|
  0 0 0 0 0 0 1 0
  ---------------
  0 0 0 0 1 1 1 0
```

## Task 9

Perform right circular shift by certain number of positions for the given number.

### Solution Details

As follows:

```
number >> count | number << TYPE_BITS_COUNT - count
```

*TYPE_BITS_COUNT* - bits count of the variable type.  
*count* - certain number of positions to be shifted by.

As you can see the idea is similar to the idea of the left circular shift.

```
  TYPE_BITS_COUNT = 8
  count = 2

  1 0 0 0 0 0 1 1
>>
                2
  ---------------
  0 0 1 0 0 0 0 0
  
  1 0 0 0 0 0 1 1
<<
                6
  ---------------
  1 1 0 0 0 0 0 0
  
  0 0 1 0 0 0 0 0
|
  1 1 0 0 0 0 0 0
  ---------------
  1 1 1 0 0 0 0 0
```

## Task 10

Find matches count of 1+ pattern (e.g. 1, 11, 111 etc.) in the number.

### Solution Details

We're taking the pattern number and performing left shift. Continuing while we reach the highest bit of the variable type. When we meet match we inc count variable. Matching determined by

```
pattern & number == pattern
```

Pattern number we get as follows:

```
1 << count - 1 | (1 << count - 1) - 1
```
*count* - consecutive ones count.

First part of *bitwise or* sets last needed bit to 1. And the second part of *bitwise or* sets all of the rest preceding bits to 1.

```
  count = 2
  
  0 1
<<  
    1
  ---
  1 0
  
  1 0
- 
    1
  ---
  0 1
  
  1 0
| 
  0 1
  ---
  1 1

  TYPE_BITS_COUNT = 8
  pattern = 1 1 (binary)

  0 1 0 0 1 1 1 0
&
  0 0 0 0 0 0 1 1
  ---------------
  0 0 0 0 0 0 1 0 == 0 0 0 0 0 0 1 1 (FALSE)

  0 1 0 0 1 1 1 0
&
  0 0 0 0 0 1 1 0
  ---------------
  0 0 0 0 0 1 1 0 == 0 0 0 0 0 1 1 0 ( TRUE) => count++
  
  0 1 0 0 1 1 1 0
&
  0 0 0 0 1 1 0 0
  ---------------
  0 0 0 0 1 1 0 0 == 0 0 0 0 1 1 0 0 ( TRUE) => count++
  
  0 1 0 0 1 1 1 0
&
  0 0 0 1 1 0 0 0
  ---------------
  0 0 0 0 1 0 0 0 == 0 0 0 1 1 0 0 0 (FALSE)
  
  0 1 0 0 1 1 1 0
&
  0 0 1 1 0 0 0 0
  ---------------
  0 0 0 0 0 0 0 0 == 0 0 1 1 0 0 0 0 (FALSE)
  
  …
  
  0 1 0 0 1 1 1 0
&
  1 1 0 0 0 0 0 0
  ---------------
  0 1 0 0 0 0 0 0 == 1 1 0 0 0 0 0 0 (FALSE)

  END
```

## Task 11

Swap two bits in the number.

### Solution Details

#### Solution 1

As follows:

```
unset_bits(number, min_index, max_index) | number >> distance & 1 << min_index | number << distance & 1 << max_index
```

*min_index* - the smaller of the two indexes.  
*max_index* - the larger of the two indexes.  
*unset_bits* - function that sets bits values at specified indexes to 0.  
*distance* - *max_index* - *min_index* (subtraction/difference).

This expression consists of three parts.  
First, we're unset bits in the specified indexes (first part).  
Then, we're find the bits values that the bits will have after the swap (second and third parts).  
And finally we're uniting three parts by *bitwise or*. Voila!

```
  min_index = 1
  max_index = 4
  distance = 4 - 1 = 3
  unset_bits(1 0 0 0 1 1, min_index, max_index) = 1 0 0 0 0 1

  0 0 0 1 0 0
&
  0 0 0 0 1 0
  -----------
  0 0 0 0 0 0
  
  0 1 1 0 0 0
&
  0 1 0 0 0 0
  -----------
  0 1 0 0 0 0
  
  1 0 0 0 0 1
|
  0 0 0 0 0 0
|
  0 1 0 0 0 0
  -----------
  1 1 0 0 0 1
```

#### Solution 2

This algorithm uses *bitwise xor* actively.

The idea of the algorithm is to put some special *swapper bit* into the positions indexes point to. And we got the *swapper* number. And then:

```
number ^ swapper
```

It will provide the bit swapping.

So, what this *swapper bit* should be? It should change or not change the  bit depending on the value of another bit. This achieved by *bitwise xor* between two bits. If bits are different, they will give 1. And 0 if they're equal. (Remember the xor definition)

```
  index_1 = 1
  index_2 = 4

GETTING THE BIT AT INDEX 1

  1 0 0 0 1 1
>>
            1
&
  0 0 0 0 0 1
  -----------
  0 0 0 0 0 1

GETTING THE BIT AT INDEX 2

  1 0 0 0 1 1
>>
            4
&
  0 0 0 0 0 1
  -----------
  0 0 0 0 0 0
  
SWAPPER BIT

  1
^
  0
  -
  1
  
THE SWAPPER

  0 1 0 0 1 0

RESULT

  1 0 0 0 1 1
^
  0 1 0 0 1 0
  -----------
  1 1 0 0 0 1
```

## Task 12

Remove bit from the number. Bits before removed bit are left untouched. Bits after removed bit are shifted to the right by 1.

### Solution Details

The algorithm is similar to algorithm in the Task 11. With the difference that we move the value of one bit and ignore the value of the other. And to get the *remover* we perform *bitwise xor* between whole number parts (not between particular bits):

```
number >> index + 1 ^ number >> index
```

Of course, after that bits should be returned to it's initial position (by performing *bitwise shift left*)

```
  index = 1

  1 0 0 0 1 1
>>
            2
  -----------
  0 0 1 0 0 0
  
  1 0 0 0 1 1
>>
            1
  -----------
  0 1 0 0 0 1
  
  0 0 1 0 0 0
^
  0 1 0 0 0 1
  -----------
  0 1 1 0 0 1

THE REMOVER

  1 1 0 0 1 0

RESULT

  1 0 0 0 1 1
^
  1 1 0 0 1 0
  -----------
  0 1 0 0 0 1
```

## Task 13

There's the collection of integer numbers. Every number in this collection except one have exactly one duplicate. But one number doesn't have duplicates. Find the value of that number.

### Solution Details

The solution is to perform *bitwise xor* between all the elements of the collection. Xor have following property:

```
x ^ x = 0
```

Besides xor have [commutative](https://en.wikipedia.org/wiki/Commutative_property) and [associative](https://en.wikipedia.org/wiki/Associative_property) properties. It means that there's no difference how elements in collection will be ordered.
