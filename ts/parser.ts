import ts, { SyntaxKind } from "typescript";
import _ from "lodash";
import {inspect} from "util";
import { trace } from "console";

const PRIMITIVE_LOOKUP = {
  [154]: "string",
  [150]: "number",
  [151]: "object",
  [136]: "boolean",
  [133]: "any",
};


const program = ts.createProgram(["ref.d.ts"], {});
const checker: ts.TypeChecker = program.getTypeChecker();

let file : ts.SourceFile | undefined = undefined;

let ROOT : ModItem  | undefined = undefined; 

for (const sourceFile of program.getSourceFiles()) {
  if((sourceFile as any).path === "/home/sammy99jsp/Projects/AvdanOS/Remote-Debug/ts/ref.d.ts") {
    ROOT = ts.forEachChild(sourceFile, visit) as any;
  }
}


function type_process(n : ts.Node) : any {
  ///@ts-ignore
  let l  = PRIMITIVE_LOOKUP[n.kind];
  if(l) {
    return { primitive : l };
  }

  if(ts.isParenthesizedTypeNode(n)) {
    return type_process(n.type);
  }

  if(ts.isUnionTypeNode(n)) {
    return {
      "union": n.types.map(a => type_process(a))
    }
  }

  if(ts.isLiteralTypeNode(n)) {
    return { "literal" : type_process(n.literal) };
  }

  if(ts.isArrayTypeNode(n)) {
    return { "array": true, ...type_process(n.elementType) }
  }

  if(ts.isTypeReferenceNode(n)) {
    return { "reference": type_process(n.typeName) }
  }

  if(ts.isIdentifier(n)) {
    return { "identifier": n.text }
  }

  if(ts.isQualifiedName(n)) {
    return { path: [n.left, n.right].flatMap(a => type_process(a)) };
  }

  if(ts.isStringLiteral(n)) {
    return n.text;
  }

  if(ts.isNumericLiteral(n)) {
    return parseFloat(n.text);
  }
  
  console.log(n)
}


function visit(node: ts.Node) {
  
  const get_doc = (n : any) =>
    ts.displayPartsToString(checker.getSymbolAtLocation(n.name)?.getDocumentationComment(checker));

    if(ts.isModuleDeclaration(node)) {
    let ident = node.name.text;
    
    let contents : any[] = [];
    
    node.body?.forEachChild(n => {
      if(ts.isModuleDeclaration(n)) {
        contents.push(visit(n));
      }

      if(ts.isInterfaceDeclaration(n)) {
        contents.push({
          type: "struct",
          ident: n.name.text,
          doc: get_doc(n),
          members: n.members.map(e => {
            return {
              ///@ts-ignore
              ident : e.name?.escapedText,
              doc: get_doc(e),
              type  : {...type_process((e as any).type), optional: !e.questionToken},
            }
          })
        })
      }

      if(ts.isEnumDeclaration(n)) {
        let doc = get_doc(n);
        let ident = n.name.text;
        let members : any[] = [];

        n.members.forEach(m => {
          let ident = m.name.getText(file);
          let doc = get_doc(m);
          let init : any = m.initializer;

          switch (init?.kind) {
            case 9: // Numeric Literal
              let k1 = (init as ts.NumericLiteral);
              init = parseFloat(k1.text);
              break;
            case 11: // String literal
              let k2 = (init as ts.StringLiteral);
              init = k2.text;
              break;
          }
          members.push({ident, doc, init});
        });

        contents.push({ type : "enum", ident, doc, members })
      }
      
      // console.log(n.kind)

      if(ts.isTypeAliasDeclaration(n)) {
        // console.log(n)
        let t = n.type;
        contents.push({
          type: "type-alias",
          ident: n.name.text,
          doc: get_doc(n),
          def: type_process(t)
        });
      } 
      
    });
    let obj = {
      type : "mod",
      doc: get_doc(node),
      ident,
      contents
    };

    return obj;
  }


  // if(ts.isInterfaceDeclaration(node)) {
  //   let n = node;
  //   let doc = ts.displayPartsToString(checker.getSymbolAtLocation(n.name)?.getDocumentationComment(checker));
  //   let d = n.members.map(m => {
  //     let doc : string | undefined = undefined;
  //     if(m.name) {
  //       doc = ts.displayPartsToString(checker.getSymbolAtLocation(m.name)?.getDocumentationComment(checker))
  //     }
  //     let n = m.name;
  //     ///@ts-ignore
  //     let t = checker.getTypeAtLocation(m.type);
      
  //     ///@ts-ignore
  //     return { ident : n?.escapedText, type : t.intrinsicName, doc };
  //   });

  //   return {
  //     ident : n.name.escapedText,
  //     doc,
  //     members: d
  //   };
  // }

}

type Type = { primitive : 'string' | 'number' | 'object' | 'boolean' | 'any', }
  | { literal : string }
  | { reference : Type, }
  | { identifier : string }
  | { union : Type[] };

type TypeModifiers = {} | { array : true } | { optional : boolean };

interface Item {
  type: 'struct' | 'enum' | 'mod' | 'type-alias',
  doc : string,
  ident: string,
}

interface StructMember {
  ident: string,
  doc: string,
  type: Type & TypeModifiers
}

interface TypeAlias extends Item {
  type: 'type-alias',
  doc  : string,
  def: Type;
}

interface EnumItem extends Item {
  type: 'enum',
  doc  : string
  members: { ident: string, doc : string, init ?: string | number } [];
}

interface StructItem extends Item {
  type: 'struct',
  members: StructMember[]
}

interface ModItem extends Item {
  type: 'mod',
  contents: Item[]
}

const rust_doc = (s ?: string) => {
  if(s == undefined || s?.length == 0) return "";
  if(s.startsWith("/**")) return s;

  return "/**\n" 
    + s.split("\n").map(ln => ` * ${ln}`).join("\n") + "\n"
    + "*/";
}

type Case = "camelCase" | "kebab-case" | "PascalCase" | "snake_case";

const CASE : Record<Case, [RegExp, RegExp, (s : string[]) => string]> = {
  "kebab-case": [
    /^[a-z][a-z0-9]*(\-[a-z][a-z0-9]*)*$/, 
    /(?<=[a-z])\-(?=[a-z])/g,
    s => s.map(p => p.toLowerCase()).join("-"),
  ],
  "snake_case": [
    /^[a-z][a-z0-9]*(\_[a-z][a-z0-9]*)*$/,
    /(?<=[a-z])\_(?=[a-z])/g,
    s => s.map(p => p.toLowerCase()).join("_"),
  ],
  "camelCase":  [
    /^[a-z]+([A-Z][a-z0-9]*)*$/,
    /(?<=[a-z])(?=[A-Z])/g,
    s => [s[0].toLowerCase(), ...s.slice(1).map(p => p[0].toUpperCase() + p.substring(1))].join("")
  ],
  "PascalCase": [
    /^([A-Z][a-zA-Z0-9]*)+$/, 
    /(?<=[a-z])(?=[A-Z])/g,
    s => s.map(p => p[0].toUpperCase() + p.substring(1)).join("")
  ],
};

const case_of = (s : string) =>
  Object.keys(CASE).find(c => CASE[c as Case][0].test(s)) as Case ?? "camelCase";

const to_case = (s : string, t: Case, f ?: Case) => {
  if(s === undefined) return "";
  return CASE[t][2](s.split(CASE[ f?? case_of(s)][1]));
};

const RUST_CONEVENTIONS = {
  enum : {
    ident   : "PascalCase",
    members : "PascalCase",
  },
  struct: {
    ident: "PascalCase",
    members: "snake_case",
  },
  "type-alias": {
    ident: "PascalCase",
  },
  mod: {
    ident: "snake_case",
  }
}

function rustify_idents(item : Item) {
  let ident_case = RUST_CONEVENTIONS[item.type].ident as Case;
  let n = item as any;
  n.ident = {from: n.ident, to : to_case(n.ident, ident_case)};

  if(n.doc) {
    n.doc = rust_doc(n.doc);
  }

  if(n.members) {
    ///@ts-ignore
    let members_case = RUST_CONEVENTIONS[item.type]?.members as Case;
    let cases = n?.members.map(({ident} : any) => case_of(ident));
    let identified_case = _.chain(cases)
      .countBy().toPairs().maxBy(_.last).head().value() as Case;

    if(identified_case !== members_case) {

      n.members = n.members.map(
        ({ident, doc, ...o} : any) => ({
          ident: {from: ident, to : to_case(ident, members_case, identified_case)},
          doc : rust_doc(n.doc),
          ...o
        }));
    }


    return n;
  }
  
  if(n.type == "mod") {
    n.contents = n.contents.map(rustify_idents);
  }
  
  return n;
}

///@ts-ignore
ROOT = rustify_idents(ROOT);

const RUST_PRIMITIVES = {
  "string": "String",
  "number": "f64",
  "boolean": "bool",
  "any": "serde_json::Value",
}

let UNIONS = {};

function find_all_unions(node : any, path : string[] = []) {
  if(node == undefined) return;
  switch (node.type) {
    case "mod":
      node.contents.map((n : any) => find_all_unions(n, [...path, node.ident.to ?? node.ident]))
      break;

    case "struct":
      node.members.map((n : any) => find_all_unions(n.type, [...path, node.ident.to ?? node.ident, n.ident.to ?? n.ident]));
      break;

    case "enum":
      node.members.map((n : any) => find_all_unions(n.type, [...path, node.ident.to ?? node.ident, n.ident.to ?? n.ident]));
      break;
    
    case "type-alias":
      find_all_unions(node.def);
      break;
  } 

  if(node.union) {
    _.set(UNIONS, path.join("."), node);
  }
}

function visit_ast(root : any, func : (n : any, p : string[]) => boolean, path : string[] = []) {
  if(root.type === "mod") {
    for (const item of root.contents) {
      if(item.type === "mod") 
        visit_ast(item, func, [...path, root.ident.to ?? root.ident,]);

      if(item.members)
        item.members.forEach((m : any) => visit_ast(m, func, [...path, root.ident.to ?? root.ident,]));

      if(func(item, [...path, root.ident.to ?? root.ident]))
        return;
    }
  }
}

function find_node(root : any, path : string | string[], current : any = []) : any {

  let p : string[];
  if(!Array.isArray(path)) {
    p = path.split(".");
  } else {
    p = path;
  }

  let new_current = [...current, root.ident.to ?? root.ident];

  let l = _.intersection(new_current, p).length;

  if(l < new_current.length) {
    return;
  }

  if(l == p.length) 
    return root;

  // console.log({p, current})


  if(root.contents) {
    for (const item of root.contents) {
      let r = find_node(item, p, new_current as any);
      if(r) return r;
    }
  }

  if(root.members) {
    for (const item of root.members) {
      let r = find_node(item, p, new_current as any);
      if(r) return r;
    }
  }
}

find_all_unions(ROOT);

import __ from "deepdash";
const {findDeep} = __(_);

function remove_unions(node : any) {
  // 1. Search for existing enums matching the union.
  visit_ast(node, (n : any, _p : string[]) => {
    if(n.type !== "enum")
      return false;

    let pa = [..._p, n.ident.to];

    let e = n as EnumItem;

    ///@ts-ignore
    let elements = e.members.map(e => e.init ?? e.ident?.from ?? e.ident);
    
    ///@ts-ignore
    let r : any = findDeep(UNIONS, (v, k, p) => {
      if(!p?.union) return;
      return p.union?.every(({literal} : any) => elements.includes(literal)) 
    });
    if(r?.parent) {
      
      let a = find_node(ROOT, r?.context.parent.path);
      delete a.type.union;

      let pp = _.cloneDeep(r?.context.parent.path.split("."));
      pp.pop();


      let s1 = pa.filter(m => !pp.includes(m));

      ///@ts-ignore
      a.type.reference = s1.join("::")
      // console.log(a)
    }
    return false;
  })
}

remove_unions(ROOT);

// console.log(inspect(ROOT?.contents[1], false, 10, true))

function display_type(t : any) : string {
  let start = "";
  let end = "";
  if(t.optional) {
    start += "Option<";
    end += ">";
  }

  if(t.array) {
    start += "Vec<";
    end += ">";
  }

  if(t.primitive) {
    ///@ts-ignore
    return start + (RUST_PRIMITIVES[t.primitive] ?? t.primitive) + end;
  }

  if(t.identifier) {
    return start + t.identifier + end;
  }

  if(t.reference) {
    return start + display_type(t.reference) + end;
  }

  if(t.path) {
    return start + display_type(t.path[0]).toLowerCase() + "::" + display_type(t.path[1]) + end;
  }

  return t;
}

function sanitize_ident(m : any) {
  if(m.ident.to === "type") {
    m.ident.to = "r#type";
  }

  return `${m.ident.to !== m.ident.from ? `#[serde(rename = "${m.ident.from}")]\n` : ""}${m.ident.to}`;
}

function assemble_rust_code(node : any) : string {

  switch (node.type) {
    case "mod":
      return `${node.doc}\npub mod ${node.ident.to} {\nuse serde::{self, Serialize, Deserialize};\nuse super::integer;\n${
        node.contents.map(assemble_rust_code).join("\n\n")
      }\n}`;
    case "struct":
      return `${node.doc}\n#[derive(Debug, Serialize, Deserialize)]\npub struct ${node.ident.to} {\n${
        node.members.map((m : any) => 
          `${m.doc}\n${sanitize_ident(m)}: ${
            display_type(m.type)
          },`  
        ).join("\n")
      }\n}`;
    
    case "enum":
      return `${node.doc}\n#[derive(Debug, Serialize, Deserialize)]\npub enum ${node.ident.to} {\n${
        node.members.map((m : any) => 
          `${m.doc}\n#[serde(rename = "${m.init ?? m.ident.from}")]\n${m.ident?.to ?? m.ident},`
        ).join("\n")
      }\n}`

    case "type-alias":
      if(!node.def?.union)
        return `${node.doc}\npub type ${node.ident.from} = ${display_type(node.def)};\n`

      let mappings = node.def.union.map(({literal} : any) => literal)
        .map((from : string) => ({from, to : to_case(from, RUST_CONEVENTIONS.enum.members as Case)}));


      return `${node.doc}\n#[derive(Debug, Serialize, Deserialize)]\npub enum ${node.ident.from} {\n${
        mappings.map((m : any) => 
          `#[serde(rename = "${m.from}")]\n${m?.to},`  
        ).join("\n")
      }\n}`;
  } 

  return "";
}

import {writeFileSync} from "fs";

writeFileSync("output.rs", assemble_rust_code(ROOT), "utf-8");
