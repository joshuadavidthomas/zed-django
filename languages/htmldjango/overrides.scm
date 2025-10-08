; Django paired blocks ({% block %}...{% endblock %})
(paired_statement) @django_block.inclusive

; Django unpaired tags ({% load %}, {% extends %}, etc.)
(unpaired_statement) @django_tag.inclusive

; Django variables ({{ variable }})
(variable) @django_variable.inclusive

; Django inline comments ({# comment #})
(unpaired_comment) @django_comment.inclusive

; Django block comments ({% comment %}...{% endcomment %})
(paired_comment) @django_comment.inclusive

; String literals
(string) @string.inclusive

; HTML content sections
(content) @html_content
