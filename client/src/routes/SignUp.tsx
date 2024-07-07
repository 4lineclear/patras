import { ChangeEvent, FormEvent, useState } from "react";

interface FormInfo {
  username: string;
  password: string;
}

const SignUp = () => {
  const [showPass, setShowPass] = useState(false);
  const [info, setInfo] = useState<FormInfo>({ username: "", password: "" });
  const infoChange = (event: ChangeEvent<HTMLInputElement>) => {
    setInfo({ ...info, [event.target.name]: event.target.value });
  };
  const signUp = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const res = await fetch("/api/req-sign-up", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(info),
    });
    switch (res.status) {
      case 200:
        break;
      case 500:
        break;
    }
    console.log(res);
  };

  return (
    <div id="sign-up-page">
      <form onSubmit={signUp}>
        <label htmlFor="username-input">Username</label>
        <input
          id="username-input"
          name="username"
          type="text"
          autoComplete="username"
          value={info.username}
          onChange={infoChange}
        />
        <label htmlFor="password-input">Password</label>
        <input
          id="password-input"
          name="password"
          type={showPass ? "text" : "password"}
          autoComplete="password"
          value={info.password}
          onChange={infoChange}
        />
        <input
          id="show-password-box"
          type="checkbox"
          checked={showPass}
          onChange={() => setShowPass((prev) => !prev)}
        />
        <span>Show password</span>
        <input type="submit" value="Sign Up" />
      </form>
    </div>
  );
};

export default SignUp;
