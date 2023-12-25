import sys
import igraph as ig


def parse_file(path):
    try:
        with open(path, 'r') as file:
            return file.readlines()
    except FileNotFoundError:
        print(f"Error: File not found at '{path}'")
    except Exception as e:
        print(f"An error occurred: {e}")


def parse_to_graph(lines):
    c_graph = ig.Graph()

    nodes = []
    edges = []
    for line in lines:
        s = line.split(":")
        node = s[0]
        nodes.append(node)

        for edge in [e for e in s[1].strip().split(" ")]:
            nodes.append(edge)
            edges.append((node, edge))

    u_nodes = list(set(nodes))
    c_graph.add_vertices(u_nodes)
    c_graph.add_edges(edges)

    c_graph.to_undirected()
    return c_graph


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Pass a file via command line", file=sys.stderr)
        sys.exit(1)

    file_path = sys.argv[1]
    lines = parse_file(file_path)
    graph = parse_to_graph(lines)

    result = graph.mincut()
    print(result)
