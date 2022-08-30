What is computation?
Input Some Data -> Compute -> Output Some Data

How to do reverse binary search tree?
function reverse(num):
	reverse[num -1] = num
	for r in reverse[num - 1]:
		reverse[num] = reverse[r]
