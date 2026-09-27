import { existsSync } from "node:fs"
import { mkdir, readFile, rm, writeFile } from "node:fs/promises"

const sirkExample = {
    tools: {
        git: {
            status: (): string[] => [],
            add: () => undefined
        },
        awaitConfirm: async () => undefined, 
        tree: (): string[] => []
    },
    agent: async (_: string, __: string): Promise<string | string[]> => ''
}

export default async function main(sirk: typeof sirkExample) {
    const docsExits = existsSync("docs")
    let files: string[] = []
    if (docsExits) {
        files = sirk.tools.git.status()
    } else {
        await mkdir("docs")
        files = sirk.tools.tree()
    }

    for (const fileName of files) {

        const docName = fileName + ".md" // manter extensão original

        const fileExists = existsSync(fileName)
        if (fileExists) {

            const fileContent = await readFile(fileName, 'utf-8')

            let explain = ''

            try {
                const docContent = await readFile(docName, 'utf-8')
                explain = await sirk.agent("code-explainer", `
                        ## Nome do arquivo: ${fileName}

                        ## Conteúdo atual do documento:
                        ${docContent}

                        ## Conteúdo do arquivo:
                        ${fileContent}`) as string
                await writeFile(docName, explain)

            } catch(error) {
                if ((error as NodeJS.ErrnoException).code === 'ENOENT') {

                    explain = await sirk.agent("code-explainer", `
                            ## Nome do arquivo: ${fileName}

                            ## Conteúdo do arquivo:
                            ${fileContent}`) as string
                    await writeFile(docName, explain)

                } else {
                    console.error('Ocorreu um erro inesperado ao ler o arquivo:', error);
                    throw error;
                }
            }

            const resume = await sirk.agent("explain-resume", explain)
            const treeExists = existsSync("docs/TREE.md")
            if (treeExists) {
                const tree = await readFile("docs/TREE.md", "utf-8")
                let updated = false
                let splitedTree = tree.split("\n")
                splitedTree = splitedTree.map((line) => {
                    if (line.startsWith(fileName + " - ")) {
                        updated = true
                        return `${fileName} - ${resume}`.trim()
                    }
                    return line.trim()
                }).filter((line) => line.length > 0)
                if (!updated) {
                    splitedTree.push(`${fileName} - ${resume}`.trim())
                }
                await writeFile("docs/TREE.md", splitedTree.toSorted().join("\n"))
            } else {
                await writeFile("docs/TREE.md", `${fileName} - ${resume}`.trim())
            }
        } else {
            if (existsSync(docName)) {
                await rm(docName)
            }

            const treeExists = existsSync("docs/TREE.md")
            if (treeExists) {
                const tree = await readFile("docs/TREE.md", "utf-8")
                let splitedTree = tree.split("\n")
                splitedTree = splitedTree.filter((line) => !line.startsWith(fileName + " - "))
                await writeFile("docs/TREE.md", splitedTree.toSorted().join("\n"))
            }
        }
    }

    await sirk.tools.awaitConfirm()

    sirk.tools.git.add()
}
