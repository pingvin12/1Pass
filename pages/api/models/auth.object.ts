export interface authObject {
  name: string;
  email: string;
  jwtToken: {
    token: string;
    validity: number;
  };
}
