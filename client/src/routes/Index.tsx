const PatrasLink = (
  <a target="_blank" href="https://en.wikipedia.org/wiki/Patras">
    Patras
  </a>
);
const GithubLink = (
  <a target="_blank" href="https://github.com/4lineclear/patras">
    Github
  </a>
);

const Index = () => (
  <div id="index">
    <br />
    <br />
    <p>PaTraS is a hyper minimalist paper trading service</p>
    <p>
      This site doesn't procure it's own data. Instead, the user must setup
      their own mechanism for getting data. This is because I'd rather not pay
      for the API.
    </p>
    <p>Here are the ways data can be inputted:</p>
    <ul>
      <li>Manual Input</li>
      <li>Automated Input</li>
    </ul>
    <br />
    <p>
      This service is named after {PatrasLink}, a Greek city dubbed "Gate to the
      West"
    </p>
    <p>See the {GithubLink}.</p>
  </div>
);

export default Index;
