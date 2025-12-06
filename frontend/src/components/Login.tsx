import { useState, type FormEvent } from "react";

export default function Login() {
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setError(null);
    setLoading(true);

    const form = event.currentTarget;
    const data = new FormData(form);

    try {
      const res = await fetch("/api/auth/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Origin: "http://localhost:3000",
          Authorization:
            "Basic " + btoa(`${data.get("username")}:${data.get("password")}`),
        },
      });

      if (res.status === 200) {
        const jsonRes = await res.json();
        console.log("jsonRes" + jsonRes);

        // store token in localstorage
        localStorage.setItem("token", jsonRes.token);

        window.location.href = "/";
      } else if (res.status === 401) {
        setError("Invalid email or password");
      } else {
        setError("An error occurred during login");
      }
    } catch (err) {
      setError("An error occurred during login");
    } finally {
      setLoading(false);
    }
  }

  return (
    <div id="login-container">
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          name="username"
          id="username"
          placeholder="Username"
          required
          disabled={loading}
        />
        <input
          type="password"
          name="password"
          id="password"
          placeholder="Password"
          required
          disabled={loading}
        />
        {error && <div className="error">{error}</div>}
        <button type="submit" disabled={loading}>
          {loading ? "Logging in..." : "Login"}
        </button>
      </form>
    </div>
  );
}
