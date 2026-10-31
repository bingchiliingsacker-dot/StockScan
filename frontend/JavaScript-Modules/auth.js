async function loginUser() {
    const username = document.getElementById("username-input-id").value;
    const password = document.getElementById("password-input-id").value;

    try {
        const response = await fetch("http://localhost:3000/api/auth/login", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ username, password }),
        });

        if (response.ok) {
            // 200: logged in
            console.log("Logged in");
            window.location.href = "inventory.html";
        } else if (response.status === 401) {
            alert("Wrong username or password");
        } else {
            alert("Server error: " + response.status);
        }
    } catch (err) {
        // network failure or CORS block
        console.error(err);
    }
}