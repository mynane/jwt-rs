export type Algorithm =
  "HS256" |
  "HS384" |
  "HS512" |
  "ES256" |
  "ES384" |
  "RS256" |
  "RS384" |
  "RS512" |
  "PS256" |
  "PS384" |
  "PS512"


export interface IClaims {
  secret: string; // secret
  aud?: string; // Optional. Audience
  exp: number; // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
  iat?: number; // Optional. Issued at (as UTC timestamp)
  iss?: string; // Optional. Issuer
  nbf?: number; // Optional. Not Before (as UTC timestamp)
  sub?: string; // Optional. Subject (whom token refers to)
  algorithm?: Algorithm; // Optional. algorithm
}

export interface IResult {
  error: boolean;
  token: string;
  message: string;
}

export type IData = {
  secret: string,
  aud: string | null,
  algorithm: string | null,
  sub: string | null,
  iss: string | null,
  iat: number | null,
  nbf: number | null,
  exp: number
}

/**
 * jwt encode
 */
declare function encode(payload: IClaims): Promise<string>;

/**
 * jwt decode
 * @param params
 */
declare function decode(token: string, secret: string): Promise<IData>;

/**
 * verify token
 * @param params
 */
declare function verify(token: string, secret: string): Promise<boolean>;

/**
 * get default validation
 */
declare function defaultValidation(): IValidation;

export { encode, decode, verify, defaultValidation };
