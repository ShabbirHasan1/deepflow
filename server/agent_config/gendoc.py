import yaml
import re

def count_leading_space(s):
    count = 0
    for c in s:
        if c == ' ':
            count += 1
        else:
            break
    return count

def get_yaml_value(yaml_dict, key_segments):
    if len(key_segments) == 1:
        return {key_segments[0]: yaml_dict.get(key_segments[0])}
    else:
        return {
            key_segments[0]: get_yaml_value(
                yaml_dict.get(key_segments[0]), key_segments[1:])
        }

# Read YAML from file
yaml_file = 'template.yaml'
with open(yaml_file, 'r', encoding='utf-8') as file:
    yaml_content = file.read()

# Match comments and yaml items
comment_pattern = re.compile(r'^\s*#(.*)$')
key_value_pattern = re.compile(r'^\s*([^#:\s]+)\s*:\s*(.*)$', re.MULTILINE)

# Extract comments
comments = {}
lines = yaml_content.split('\n')
current_keys = []
current_comment = []

for line in lines:
    comment_match = comment_pattern.match(line)
    key_value_match = key_value_pattern.match(line)

    if comment_match:
        current_comment.append(comment_match.group(1))
    elif key_value_match:
        current_keys = current_keys[:count_leading_space(line)//2]
        current_keys.append(key_value_match.group(1))
        comments['.'.join(current_keys)] = '\n'.join(current_comment)
        current_comment = []
    else:
        current_comment = []
        continue

# Load Yaml items
data = yaml.safe_load(yaml_content)

# Generate doc
for key, comment in comments.items():
    segments = key.split('.')

    try:
        parsed_comment = yaml.safe_load(comment)
    except yaml.YAMLError as e:
        print(f"Error parsing YAML comment at {key}: {e}")

    title = '#' * len(segments)
    if not parsed_comment:
        parent_comment = comments.get('.'.join(segments[:-1]), '')
        parent_parsed_comment = yaml.safe_load(parent_comment)
        parent_type = (parent_parsed_comment or {}).get('type', '')
        if parent_type in ['section', '']:
            print(f"<mark>ERROR: yaml item with no comment, {key}</mark>\n")
        continue
    if  not parsed_comment.get('type'):
        print(f"<mark>ERROR: yaml item with comment but has no `type` attribute, {key}</mark>\n")
        continue
    item_type = parsed_comment.get('type')
    name = parsed_comment.get('name', '[FIXME]')
    desc = parsed_comment.get('description', '')
    print(f"{title} {name} " + "{#" + key + "}\n")
    if item_type == 'section':
        if desc:
            print(f"{desc}\n")
    else:
        # Tags
        modification = parsed_comment.get('modification', '')
        print("**Tags**:\n")
        if modification != 'hot_update':
            print(f"<mark>{modification}</mark>")
        else:
            print(f"`{modification}`")
        ee_feature = parsed_comment.get('ee_feature', False)
        if ee_feature:
            print(f"<mark>ee_feature</mark>")
        deprecated = parsed_comment.get('deprecated', False)
        if deprecated:
            print(f"<mark>deprecated</mark>")
        print("")
        # FQCN
        upgrade_from = parsed_comment.get('upgrade_from', [])
        print("**FQCN**:\n")
        print(f"`{key}`\n")
        print(f"Upgrade from version <= 6.5.9: `{upgrade_from}`\n")
        # Default value
        yaml_example = yaml.dump(get_yaml_value(data, segments), default_flow_style=False)
        print("**Default value**:")
        print("```yaml")
        print(f"{yaml_example}```\n")
        # Enum options
        enum_options = parsed_comment.get('enum_options')
        if enum_options:
            print("**Enum options**:")
            print("| Key  | Name                         |")
            print("| ---- | ---------------------------- |")
            for eo in enum_options:
                if isinstance(eo, dict):
                    eok = list(eo.keys())[0]
                    eov = list(eo.values())[0]
                    print(f"| {eok} | {eov} |")
                else:
                    print(f"| {eo} | |")
            print("")
        # Schema
        unit = parsed_comment.get('unit', '')
        value_range = parsed_comment.get('range', [])
        print("**Schema**:")
        print("| Key  | Value                        |")
        print("| ---- | ---------------------------- |")
        print(f"| Type | {item_type} |")
        if unit:
            print(f"| Unit | {unit} |")
        if value_range:
            print(f"| Range | {value_range} |")
        print("")
        # Description
        if desc:
            print(f"**Description**:\n")
            print(f"{desc}\n")
