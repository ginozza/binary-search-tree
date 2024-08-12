use std::fmt; // Importa el módulo fmt para la funcionalidad de formateo.

// Definición de una estructura llamada `Node`.
struct Node {
    value: i32, // Tipo entero de 32 bits para el valor del nodo.
    left: Option<Box<Node>>, // `Option` es un tipo enum que puede ser `Some` o `None`. `Box` se usa para almacenar datos en el heap.
    right: Option<Box<Node>>, // Similar a `left`, para el hijo derecho.
}

// Definición de una estructura llamada `BST` (Binary Search Tree).
struct BST {
    root: Option<Box<Node>>, // La raíz del árbol, que puede ser `None` si está vacío.
    len: Option<i32>, // Número de nodos en el árbol, también `Option` para manejar casos donde no se cuenta.
}

// Implementación del trait `fmt::Display` para `Node`.
// Esto permite imprimir la estructura `Node` utilizando el macro `println!`.
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value) // Escribe el valor del nodo en el buffer de salida.
    }
}

// Implementación del trait `fmt::Display` para `BST`.
impl fmt::Display for BST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sb = String::new(); // `String::new()` crea una nueva cadena vacía.
        self.in_order_traversal(&mut sb, &self.root); // Llama a un método interno para recorrer el árbol.
        write!(f, "{}", sb.trim()) // Escribe la cadena resultante sin espacios al final.
    }
}

// Implementación de métodos para la estructura `BST`.
impl BST {
    // Método para recorrer el árbol en orden.
    fn in_order_traversal(&self, sb: &mut String, root: &Option<Box<Node>>) {
        if let Some(node) = root { // `if let` permite desempacar un `Option`.
            self.in_order_traversal(sb, &node.left); // Llamada recursiva para el subárbol izquierdo.
            sb.push_str(&format!("{} ", node)); // Añade el valor del nodo actual a la cadena.
            self.in_order_traversal(sb, &node.right); // Llamada recursiva para el subárbol derecho.
        }
    }

    // Método para añadir un nuevo valor al árbol.
    fn add(&mut self, value: i32) {
        let root = self.root.take(); // `take()` toma el valor de `Option` y lo reemplaza por `None`.
        self.root = self.add_by_node(root, value); // Inserta el nuevo valor en la posición correcta.
        if let Some(len) = self.len.as_mut() { // `as_mut()` permite modificar el valor dentro de `Option`.
            *len += 1; // Incrementa el contador de nodos.
        }
    }

    // Método auxiliar que realiza la inserción de un nodo en el árbol.
    fn add_by_node(&self, root: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
        match root { // `match` es similar a `switch` en otros lenguajes.
            Some(mut node) => { // Si el nodo existe (`Some`), compara e inserta recursivamente.
                if value < node.value {
                    node.left = self.add_by_node(node.left, value); // Inserción en el subárbol izquierdo.
                } else {
                    node.right = self.add_by_node(node.right, value); // Inserción en el subárbol derecho.
                }
                Some(node) // Devuelve el nodo actualizado.
            }
            None => Some(Box::new(Node { value, left: None, right: None })), // Crea un nuevo nodo si no existe.
        }
    }

    // Método para buscar un valor en el árbol.
    fn search(&self, value: i32) -> (Option<&Node>, bool) {
        self.search_by_node(&self.root, value) // Llama a un método auxiliar para realizar la búsqueda.
    }

    // Método auxiliar para buscar un valor en el árbol.
    fn search_by_node<'a>(&self, root: &'a Option<Box<Node>>, value: i32) -> (Option<&'a Node>, bool) {
        match root { // `match` para manejar las diferentes condiciones de búsqueda.
            Some(node) => {
                if value == node.value {
                    (Some(&node), true) // Valor encontrado.
                } else if value < node.value {
                    self.search_by_node(&node.left, value) // Búsqueda en el subárbol izquierdo.
                } else {
                    self.search_by_node(&node.right, value) // Búsqueda en el subárbol derecho.
                }
            }
            None => (None, false), // Valor no encontrado.
        }
    }

    // Método para eliminar un valor del árbol.
    fn remove(&mut self, value: i32) {
        let root = self.root.take(); // Toma el valor de la raíz y la reemplaza con `None`.
        self.root = self.remove_by_node(root, value); // Llama a un método auxiliar para realizar la eliminación.
    }

    // Método auxiliar para eliminar un valor del árbol.
    fn remove_by_node(&self, root: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
        match root {
            Some(mut node) => {
                if value < node.value {
                    node.left = self.remove_by_node(node.left, value); // Elimina en el subárbol izquierdo.
                    Some(node)
                } else if value > node.value {
                    node.right = self.remove_by_node(node.right, value); // Elimina en el subárbol derecho.
                    Some(node)
                } else {
                    // Nodo a eliminar encontrado
                    if node.left.is_none() {
                        node.right // Si solo tiene hijo derecho, retorna ese hijo.
                    } else if node.right.is_none() {
                        node.left // Si solo tiene hijo izquierdo, retorna ese hijo.
                    } else {
                        // Nodo con dos hijos
                        let mut min_larger_node = node.right.take(); // Toma el subárbol derecho para buscar el menor valor.
                        let min_value = loop {
                            match min_larger_node {
                                Some(mut min_node) => {
                                    if min_node.left.is_none() {
                                        break min_node.value; // Encuentra el valor mínimo.
                                    }
                                    min_larger_node = min_node.left.take(); // Continúa buscando.
                                }
                                None => break -1, // Caso improbable.
                            }
                        };
                        node.value = min_value; // Reemplaza el valor del nodo con el valor mínimo encontrado.
                        node.right = self.remove_by_node(node.right, min_value); // Elimina el nodo con el valor mínimo.
                        Some(node) // Devuelve el nodo actualizado.
                    }
                }
            }
            None => None, // Si el nodo no existe, retorna `None`.
        }
    }
}

fn main() {
    // Crear un árbol binario de búsqueda con tres nodos.
    let mut n = Box::new(Node { value: 1, left: None, right: None });
    n.left = Some(Box::new(Node { value: 0, left: None, right: None }));
    n.right = Some(Box::new(Node { value: 2, left: None, right: None }));

    let len_value = 3; // Cantidad de nodos en el árbol.
    let mut b = BST {
        root: Some(n),
        len: Some(len_value),
    };

    println!("{}", b); // Imprime la representación del árbol.
    b.add(4); // Añade un nodo con el valor `4`.
    b.add(5); // Añade un nodo con el valor `5`.
    b.add(6); // Añade un nodo con el valor `6`.
    println!("{}", b); // Imprime el árbol después de las inserciones.

    let (found, exists) = b.search(6); // Busca el valor `6` en el árbol.
    println!("Found 6: {}, Exists: {}", found.map_or("None".to_string(), |n| n.to_string()), exists);

    let (found, exists) = b.search(3); // Busca el valor `3`, que no existe.
    println!("Found 3: {}, Exists: {}", found.map_or("None".to_string(), |n| n.to_string()), exists);

    b.remove(6); // Elimina el valor `6` del árbol.
    println!("{}", b); // Imprime el árbol después de la eliminación.
    b.remove(3); // Intenta eliminar el valor `3`, que no existe.
    println!("{}", b); // Imprime el árbol nuevamente.
}