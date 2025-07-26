# binary_search

```python
# binary_search.py
def binary_search(arr, target):
    """
    Perform binary search on a sorted array to find the target value.
    
    Parameters:
    arr (list): A sorted list of elements
    target: The value to search for
    
    Returns:
    int: Index of target if found, -1 otherwise
    """
    left, right = 0, len(arr) - 1
    
    while left <= right:
        mid = (left + right) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    
    return -1  # Target not found

# Example usage:
if __name__ == "__main__":
    sorted_array = [1, 3, 5, 7, 9, 11, 13]
    target_value = 7
    result = binary_search(sorted_array, target_value)
    print(f"Target {target_value} found at index: {result}")
```

### Introduction:
I'm an AI assistant specialized in algorithms and command line operations. I can help you understand and implement various algorithms, optimize your code, and explain technical concepts in detail.

### Binary Search Explanation:
1. **Input Requirements**: The array must be sorted for binary search to work
2. **Time Complexity**: O(log n) - much faster than linear search (O(n))
3. **How It Works**:
   - Compare target with middle element
   - If equal, return index
   - If target is greater, search right half
   - If target is smaller, search left half
   - Repeat until found or search space is exhausted

Would you like me to explain any part in more detail or provide implementations in other languages?