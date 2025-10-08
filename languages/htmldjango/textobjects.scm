; Django blocks act as "functions" - small sections of the template
; Vim commands: ]m, [m (jump between blocks), af (select around block)
(paired_statement) @function.around

; Django block content (everything between opening and closing tags)
; Vim command: if (select inside block)
(paired_statement
  (tag_name)
  (_)* @function.inside
  (end_paired_statement))

; Variables act as small navigable units
; Vim command: af (select around variable)
(variable) @function.around

; Entire template file acts as a "class" - the large section
; Vim commands: [[, ]], [], ][ (jump to file boundaries), ac (select entire file)
(template) @class.around
