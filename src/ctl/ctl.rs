use crate::args::Cli;

use super::error::PongaError;
pub(crate) struct Ctl {
    args: Cli,
}

impl Ctl {
    pub fn new(args: Cli) -> Self {
        Ctl {
            args,
        }
    }
    pub async fn connect(&self) -> Result<(), PongaError> {

        // waiting for the auth part to be done on the server-side...

        // 1. authentication (TODO)
            // pongactl discovers the authentication method (password, oidc, ...) and then opens
            // a browser window with a login page

            // 1.2. oidc callback (TODO)
                // if oidc: 
                // meanwhile, a temporary webserver has been started: this will be used to collect the authorization code from an oidc provider
                // and fulfill the oidc authorization flow
            // 1.3. authentication polling - password (TODO)
                // if password: 
                // pongactl issues a password authentication request, creating a request id on pongaok. 
                // such id will be used to poll the authentication status (i.e. waiting for the user to login)
            // 1.4. jwt 
                // when 1.2/1.3 has completed, pongaok will issue a jwt that will be used to connect to the tunnel.
                // it's now up to the tunnel entry (pongad) to verify and authenticate the request!
    
        // 2. tunnel setup (TODO)
            // assuming the authentication happened successfully, the tunnel will be setup and a secure connection
            // will be estabilished. ssh will be tunneled over websockets!

        Ok(())
    }
}