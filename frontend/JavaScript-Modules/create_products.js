const productName = document.getElementById("product-name");
const productId = document.getElementById("product-id");
const price = document.getElementById("product-price");
const stock = document.getElementById("product-stock");

const result = document.getElementById("result");

async function createProducts() {
    if (productName.value.trim() === "") {
        throw new Error("Product name cannot be empty");
    }

    if (productName.value === "" || productId.value === "" || price.value === "" || stock.value === "") {
        throw new Error("Please give all required fields");
    }

    const response = await fetch("http://localhost:3000/api/products");

    if (!response.ok) {
        throw new Error("Failed to retrieve products");
    }

    const products = await response.json();

    // Checks if product name or ID already exists
    if (products
        .some(
            product =>
                product.product_name.trim().toLowerCase() === productName.value.trim().toLowerCase() ||
                Number(product.product_id) === Number(productId.value)
        )
    ) {
        throw new Error("Product name/ID already exists");
    }

    const newProduct = {
        product_id: Number(productId.value),
        product_name: productName.value.trim(),
        price: Number(Number(price.value).toFixed(2)),
        stock: Number(stock.value),
    };

    const postResponse = await fetch("http://localhost:3000/api/products", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(newProduct),
    });

    if (!postResponse.ok) {
        const errData = await postResponse.json();
        throw new Error(errData.message);
    }

    return await postResponse.json();

}

async function clearAllInputs() {
    productName.value = "";
    productId.value = "";
    price.value = "";
    stock.value = "";
}

async function handleCreateProduct() {
    try {
        const createdProduct = await createProducts();

        result.textContent = `Created product: ${createdProduct.product_name}!`;

        window.location.href = "inventory.html";
    } catch (error) {
        result.textContent = error.message;
    }
}
