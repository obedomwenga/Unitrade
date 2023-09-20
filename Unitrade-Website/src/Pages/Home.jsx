import React from "react";
import Nav from "../components/Nav";
import Page from "../components/Page";
import Footer from "../components/Footer";
import Help from "../components/Help";

function Home() {
  return (
    <div>
      <Nav />
      <Page />
      <Help />
      <Footer />
    </div>
  );
}

export default Home;