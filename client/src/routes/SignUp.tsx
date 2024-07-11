import { ChangeEvent, FormEvent, useState } from "react";
import styles from "./sign-up.module.scss";

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
        alert("User added");
        break;
      case 400:
        alert("Invalid password inputted");
        break;
      case 409:
        alert("Username taken");
        break;
      case 500:
        alert("Internal server error");
        break;
    }
    console.log(res);
  };

  return (
    <div id={styles.page}>
      <form id={styles.form} onSubmit={signUp}>
        <h2>Create a patras account</h2>
        <p>Complete the form below to create your account</p>

        <div id={styles.usernameLabelDiv}>
          <label className={styles.label} htmlFor="username-input">
            Username:
          </label>
        </div>
        <input
          id={styles.usernameInput}
          className={styles.input}
          name="username"
          type="text"
          autoComplete="username"
          value={info.username}
          onChange={infoChange}
        />
        <div id={styles.passwordLabelDiv}>
          <label className={styles.label} htmlFor="password-input">
            Password:
          </label>
          <input
            id={styles.showPasswordInput}
            type="checkbox"
            checked={showPass}
            tabIndex={-1}
            onChange={() => setShowPass((prev) => !prev)}
          />
          <span id={styles.showPasswordSpan}>Show</span>
        </div>
        <input
          id={styles.passwordInput}
          className={styles.input}
          name="password"
          type={showPass ? "text" : "password"}
          autoComplete="password"
          value={info.password}
          onChange={infoChange}
        />
        <input id={styles.submitButton} type="submit" value="Sign Up" />
      </form>
    </div>
  );
};

export default SignUp;
