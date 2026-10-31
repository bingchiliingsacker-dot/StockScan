// Global Variables
let inEdit = false;

async function filterProducts(input, exactId = false) {
    let query = input.value.trim().toLowerCase();

    const response = await fetch(`http://localhost:3000/api/products`);

    if (!response.ok) {
        return;
    }

    let output = [];

    let products = await response.json();

    if (exactId) {
        let exactMatch = products.find(
            product => String(product.product_id) === query
        )

        if (exactMatch) {
            return [exactMatch];
        }
    }


    for (let product of products) {
        if (product.product_name.trim().toLowerCase().includes(query)) {
            output.push(product);
        }
    }

    return output;
}

async function searchProduct(mode) {
    const query = document.querySelector(".search-box").value.trim();

    let response;
    let product;

    if (!["id", "name"].includes(mode)) {
        document.getElementById("result").textContent =
            "Invalid mode";
        return { success: false, product: null, status: null };
    }

    if (query === "") {
        document.getElementById("result").textContent =
            "Please enter a product name or ID.";
        return { success: false, product: null, status: 400 };
    }

    function checkResponse(response) {
        if (!response.ok) {
            const error = new Error(`HTTP error: ${response.status}`);
            error.status = response.status;
            throw error;
        }
    }

    try {
        if (mode === "id") {
            response = await fetch(
                `http://127.0.0.1:3000/api/products/${encodeURIComponent(query)}`
            );

            checkResponse(response);

            product = await response.json();

            return { success: true, product: product, status: 200 };
        } else if (mode === "name") {
            response = await fetch(
                `http://127.0.0.1:3000/api/products`
            );

            checkResponse(response);

            const products = await response.json();

            const list = products
                .filter(p => p.product_name.toLowerCase().includes(query.toLowerCase()))
                .map(p => `${p.product_name} | ID: ${p.product_id} | Price: ${p.price} | Stock: ${p.stock}`)
                .join("\n");

            if (!list) {
                document.getElementById("result").textContent = "Product not found";
                return { success: false, product: null, status: 404 };
            }

            document.getElementById("result").textContent = list;

            return { success: true, product: list, status: 200 };
        }
    }
    catch (error) {
        document.getElementById("result").textContent = `Error: ${error.message}`;

        return { success: false, product: null, status: error.status ?? null };
    }
}

async function deleteProduct(productId, productName) {
    const confirmed = confirm("Are you sure you want to delete this product?");

    if (!confirmed) {
        return;
    }

    try {
        const response = await fetch(
            `http://127.0.0.1:3000/api/products/${encodeURIComponent(productId)}`
            , {
                method: "DELETE",
            }
        )

        if (!response.ok) {
            throw new Error(`HTTP error: ${response.status}`);
        }

        document.getElementById("result").textContent =
            `Deleted ${productName} successfully!`;

        let input = document.getElementById("search-box-input");

        await displayProducts(input);
    } catch (error) {
        document.getElementById("result").textContent =
            `Failed to delete ${productName}: ${error.message}`;
    }
}

async function saveProduct(productId, updatedProduct) {
    const response = await fetch(
        `http://127.0.0.1:3000/api/products/${encodeURIComponent(productId)}`,
        {
            method: "PUT",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(updatedProduct)
        }
    );

    if (!response.ok) {
        throw new Error(`HTTP error: ${response.status}`);
    }
}

async function displayProducts(input, exactId = false) {
    function editProduct(row, p) {
        if (inEdit) return;

        inEdit = true;
        row.dataset.editing = "true";

        const cells = row.querySelectorAll("td");

        const original = {
            product_id: p.product_id,
            product_name: p.product_name,
            price: p.price,
            stock: p.stock,
        }

        function createInput(value, type = "text") {
            const input = document.createElement("input");
            input.type = type;
            input.value = value;
            input.classList.add("product-input");
            return input;
        }

        const nameInput = createInput(original.product_name);

        const idInput = createInput(original.product_id, "number");
        idInput.readOnly = true;

        const priceInput = createInput(original.price, "number");
        priceInput.step = "0.01";
        priceInput.min = "0";

        const stockInput = createInput(original.stock, "number");
        stockInput.step = "1";
        stockInput.min = "0";

        const actionsCell = row.lastElementChild;

        const cancelButton = document.createElement("button");
        cancelButton.type = "button";
        cancelButton.className = "button-configs";
        cancelButton.textContent = "✕";

        const saveButton = document.createElement("button");
        saveButton.type = "button";
        saveButton.className = "button-configs";
        saveButton.textContent = "💾";

        actionsCell.replaceChildren(cancelButton, saveButton);

        // Keep the product ID unchanged.
        cells[0].replaceChildren(nameInput);
        cells[1].replaceChildren(idInput);
        cells[2].replaceChildren(priceInput);
        cells[3].replaceChildren(stockInput);

        cancelButton.addEventListener("click", async () => {
            // Restore the original cell contents.
            row.dataset.editing = "false";
            inEdit = false;
            await displayProducts(input);
        });

        saveButton.addEventListener("click", async () => {
            // Validate inputs before saving.
            const inputs = [nameInput, idInput, priceInput, stockInput];

            if (!inputs.every(input => input.checkValidity())) {
                alert("Please enter valid values.");
                return;
            }

            if (!nameInput.value.trim()) {
                alert("Product name cannot be empty.");
                return;
            }

            const updatedProduct = {
                product_id: original.product_id,
                product_name: nameInput.value.trim(),
                price: Number(priceInput.value),
                stock: Number(stockInput.value)
            };

            try {
                await saveProduct(original.product_id, updatedProduct);
                inEdit = false;
                await displayProducts(input);
            } catch (error) {
                console.error(error);
                alert("Failed to save product.");
            }
        });
    }

    let products = await filterProducts(input, exactId);

    const rows = document.getElementById("product-rows");

    rows.replaceChildren();

    products.forEach(p => {
        const row = document.createElement("tr");

        [p.product_name, p.product_id, p.price, p.stock].forEach(value => {
            const cell = document.createElement("td");
            cell.textContent = value;
            row.appendChild(cell);
        });

        // Create the Actions cell
        const actionsCell = document.createElement("td");

        // Create the delete button
        const deleteButton = document.createElement("button");
        deleteButton.type = "button";
        deleteButton.className = "button-configs";
        deleteButton.setAttribute("aria-label", `Delete ${p.product_name}`);
        deleteButton.addEventListener("click", () => {
            deleteProduct(p.product_id, p.product_name);
        });

        // Display the decal
        const decal = document.createElement("img");
        decal.src = "../images/delete-icon.png"; // Change if your filename differs
        decal.alt = "";
        decal.width = 20;
        decal.height = 20;

        // Assemble the elements
        deleteButton.appendChild(decal);
        actionsCell.appendChild(deleteButton);
        row.appendChild(actionsCell);

        const editButton = document.createElement("button");
        editButton.type = "button";
        editButton.className = "button-configs";
        editButton.setAttribute("aria-label", `Edit ${p.product_name}`);

        editButton.addEventListener("click", () => {
            editProduct(row, p);
        });

        const editIcon = document.createElement("img");
        editIcon.src = "../images/edit-icon.png";
        editIcon.alt = "";
        editIcon.width = 20;
        editIcon.height = 20;

        editButton.appendChild(editIcon);
        actionsCell.appendChild(editButton);

        rows.appendChild(row);
    });
}

window.onload = function () {
    const searchBox = document.getElementById("search-box-input");

    displayProducts(searchBox).then(r => 0);

    searchBox.addEventListener("input", async (e) => {
        let input = e.target;
        await displayProducts(input);
    });
};
