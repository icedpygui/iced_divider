# iced_divider
An Iced widget used to change the size of adjacent containers using the mouse horizontally or vertically.

<div align="center">
  
https://github.com/user-attachments/assets/430cf3d2-660b-4c07-9a23-d7445e43ff1a

</div>

## Easy to Use

* Using a stack, place the divider over your containers.
* Uses the values of the widths or heights to place the handle over the edge of the container.
* Resize the containers using the return message when the handle is moved.
* The message routine is simple, see examples, and is esssentially the same for all situations.
* Make a table easily, you have full control of the content and styling.
* Tables can have headers or footers, no restrictions.
* Note on padding:  
    Padding cannot be used on the outer container in the stack because the divider
    can't detect whether padding is used causing alignment issues.  Therefore, 
    place the stack in a container and apply padding as needed.

## Future enhancements for v0.4.0
add in a container to eliminate the stack
