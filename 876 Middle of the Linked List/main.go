package main

func main() {
	middleNode(
		&ListNode{
			Val: 1,
			Next: &ListNode{
				Val: 2,
				Next: &ListNode{
					Val: 3,
					Next: &ListNode{
						Val: 4,
						Next: &ListNode{
							Val: 5,
						},
					},
				},
			},
		},
	)
}

type ListNode struct {
	Val  int
	Next *ListNode
}

func middleNode(head *ListNode) *ListNode {
	allNodes := []*ListNode{}
	current := head
	for {
		allNodes = append(allNodes, current)

		next := current.Next
		if next != nil {
			current = next
		} else {
			break
		}
	}

	return allNodes[len(allNodes)/2]
}
