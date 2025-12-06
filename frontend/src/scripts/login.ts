export async function login(event: SubmitEvent) {
  event.preventDefault();
  const form = event.target as HTMLFormElement;
  const data = new FormData(form);

  localStorage.setItem("token", "pending");

  let res = await fetch("/api/login", {
    method: "POST",
    body: JSON.stringify(Object.fromEntries(data)),
    headers: { "Content-Type": "application/json" },
  });
  let jsonRes = await res.json();
  if (jsonRes.data.success) {
    alert("Login successful");
    window.location.href = "/";
  } else {
    localStorage.removeItem("token");
    alert("Invalid email or password");
  }
}
export default login;
