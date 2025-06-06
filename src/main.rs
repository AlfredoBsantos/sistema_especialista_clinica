use std::collections::HashMap;
use std::io;

fn main() {
    println!("--- Sistema Especialista: Triagem de Sintomas ---");
    println!("Olá! Qual é o seu nome?");

    let mut nome = String::new();
    io::stdin().read_line(&mut nome).expect("Falha ao ler o nome");
    let nome = nome.trim().to_string();

    // HashMap para armazenar todos os "fatos" conhecidos pelo sistema.
    // Chave: nome da variável (String), Valor: valor da variável (String).
    let mut fatos_conhecidos: HashMap<String, String> = HashMap::new();

    // 1. Coleta de Variáveis de Entrada

    // Temperatura Corporal
    println!("\n{}, por favor, digite sua temperatura corporal (normal, febril_moderado, febril_alto):", nome);
    let mut temp_input = String::new();
    io::stdin().read_line(&mut temp_input).expect("Falha ao ler a temperatura");
    fatos_conhecidos.insert("Temperatura_Corporal".to_string(), temp_input.trim().to_string());

    // Dor Localização
    println!("Onde você sente dor? (cabeca, garganta, abdomen, peito, articulacao, nenhuma):");
    let mut dor_localizacao_input = String::new();
    io::stdin().read_line(&mut dor_localizacao_input).expect("Falha ao ler a localização da dor");
    fatos_conhecidos.insert("Dor_Localizacao".to_string(), dor_localizacao_input.trim().to_string());

    // Dor Intensidade
    println!("Qual a intensidade da dor? (leve, moderada, forte):");
    let mut dor_intensidade_input = String::new();
    io::stdin().read_line(&mut dor_intensidade_input).expect("Falha ao ler a intensidade da dor");
    fatos_conhecidos.insert("Dor_Intensidade".to_string(), dor_intensidade_input.trim().to_string());

    // Sintomas Respiratórios (separe por vírgula)
    println!("Quais sintomas respiratórios você tem? (tosse, falta_de_ar, nariz_entupido, dor_garganta, nenhum - separe por vírgula):");
    let mut sintomas_respiratorios_input = String::new();
    io::stdin().read_line(&mut sintomas_respiratorios_input).expect("Falha ao ler os sintomas respiratórios");
    fatos_conhecidos.insert("Sintomas_Respiratorios".to_string(), sintomas_respiratorios_input.trim().to_string());

    // Sintomas Gastrointestinais (separe por vírgula)
    println!("Quais sintomas gastrointestinais você tem? (nausea_vomito, diarreia, dor_estomago, nenhum - separe por vírgula):");
    let mut sintomas_gastrointestinais_input = String::new();
    io::stdin().read_line(&mut sintomas_gastrointestinais_input).expect("Falha ao ler os sintomas gastrointestinais");
    fatos_conhecidos.insert("Sintomas_Gastrointestinais".to_string(), sintomas_gastrointestinais_input.trim().to_string());

    // Sinais de Alerta Graves (separe por vírgula)
    println!("Você teve algum sinal de alerta grave? (perda_consciencia, sangramento_intenso, dor_no_peito_irradiando, nenhum):");
    let mut sinais_alerta_graves_input = String::new();
    io::stdin().read_line(&mut sinais_alerta_graves_input).expect("Falha ao ler os sinais de alerta graves");
    fatos_conhecidos.insert("Sinais_Alerta_Graves".to_string(), sinais_alerta_graves_input.trim().to_string());

    // Histórico Médico Relevante (separe por vírgula)
    println!("Você tem algum histórico médico relevante? (asma_cronica, diabetes, cardiaco, nenhum - separe por vírgula):");
    let mut historico_medico_relevante_input = String::new();
    io::stdin().read_line(&mut historico_medico_relevante_input).expect("Falha ao ler o histórico médico relevante");
    fatos_conhecidos.insert("Historico_Medico_Relevante".to_string(), historico_medico_relevante_input.trim().to_string());

    println!("\n--- Fatos Coletados ---");
    for (chave, valor) in &fatos_conhecidos {
        println!("{}: {}", chave, valor);
    }
    println!("-----------------------");

    // 2. Motor de Inferência (Progressivo - Forward Chaining)
    // As inferências são adicionadas ao `fatos_conhecidos` conforme são deduzidas.

    // --- VARIÁVEIS INTERMEDIÁRIAS - NÍVEL 1 ---

    // Indicador_Febre_Alta
    let temp_corporal = fatos_conhecidos.get("Temperatura_Corporal").map_or("".to_string(), |s| s.to_string());
    let mut indicador_febre_alta = "não".to_string();
    if temp_corporal == "febril_alto" {
        indicador_febre_alta = "sim".to_string();
        println!("  Inferência Nível 1: (Febre Alta) -> Indicador_Febre_Alta = Sim");
    }
    fatos_conhecidos.insert("Indicador_Febre_Alta".to_string(), indicador_febre_alta.clone());

    // Sindrome_Respiratoria_Suspeita
    let sintomas_resp = fatos_conhecidos.get("Sintomas_Respiratorios").map_or("".to_string(), |s| s.to_string());
    let mut sindrome_respiratoria_suspeita = "não".to_string();
    if sintomas_resp.contains("tosse") || sintomas_resp.contains("falta_de_ar") || sintomas_resp.contains("nariz_entupido") || sintomas_resp.contains("dor_garganta") {
        sindrome_respiratoria_suspeita = "sim".to_string();
        println!("  Inferência Nível 1: (Sintomas Respiratórios) -> Sindrome_Respiratoria_Suspeita = Sim");
    }
    fatos_conhecidos.insert("Sindrome_Respiratoria_Suspeita".to_string(), sindrome_respiratoria_suspeita.clone());

    // Sindrome_Gastrointestinal_Suspeita
    let sintomas_gastro = fatos_conhecidos.get("Sintomas_Gastrointestinais").map_or("".to_string(), |s| s.to_string());
    let mut sindrome_gastrointestinal_suspeita = "não".to_string();
    if sintomas_gastro.contains("nausea_vomito") || sintomas_gastro.contains("diarreia") || sintomas_gastro.contains("dor_estomago") {
        sindrome_gastrointestinal_suspeita = "sim".to_string();
        println!("  Inferência Nível 1: (Sintomas Gastrointestinais) -> Sindrome_Gastrointestinal_Suspeita = Sim");
    }
    fatos_conhecidos.insert("Sindrome_Gastrointestinal_Suspeita".to_string(), sindrome_gastrointestinal_suspeita.clone());

    // Indicador_Dor_Aguda
    let dor_intensidade = fatos_conhecidos.get("Dor_Intensidade").map_or("".to_string(), |s| s.to_string());
    let dor_localizacao = fatos_conhecidos.get("Dor_Localizacao").map_or("".to_string(), |s| s.to_string());
    let mut indicador_dor_aguda = "não".to_string();
    if dor_intensidade == "forte" && dor_localizacao != "nenhuma" {
        indicador_dor_aguda = "sim".to_string();
        println!("  Inferência Nível 1: (Dor Forte) -> Indicador_Dor_Aguda = Sim");
    }
    fatos_conhecidos.insert("Indicador_Dor_Aguda".to_string(), indicador_dor_aguda.clone());

    // Indicador_Alerta_Grave
    let sinais_alerta = fatos_conhecidos.get("Sinais_Alerta_Graves").map_or("".to_string(), |s| s.to_string());
    let mut indicador_alerta_grave = "não".to_string();
    if sinais_alerta.contains("perda_consciencia") || sinais_alerta.contains("sangramento_intenso") || sinais_alerta.contains("dor_no_peito_irradiando") {
        indicador_alerta_grave = "sim".to_string();
        println!("  Inferência Nível 1: (Sinal de Alerta Grave) -> Indicador_Alerta_Grave = Sim");
    }
    fatos_conhecidos.insert("Indicador_Alerta_Grave".to_string(), indicador_alerta_grave.clone());

    println!("\n--- Inferências Nível 1 Concluídas ---");

    // --- VARIÁVEIS INTERMEDIÁRIAS - NÍVEL 2 ---

    // Categoria_Problema_Saude
    let mut categoria_problema_saude = "Nenhum".to_string();
    if sindrome_respiratoria_suspeita == "sim" && indicador_febre_alta == "sim" {
        categoria_problema_saude = "Infeccao_Respiratoria_Grave".to_string();
        println!("  Inferência Nível 2: (Síndrome Respiratória + Febre Alta) -> Categoria_Problema_Saude = Infecção Respiratória Grave");
    } else if sindrome_respiratoria_suspeita == "sim" {
        categoria_problema_saude = "Infeccao_Respiratoria_Leve".to_string();
        println!("  Inferência Nível 2: (Síndrome Respiratória) -> Categoria_Problema_Saude = Infecção Respiratória Leve");
    } else if sindrome_gastrointestinal_suspeita == "sim" {
        categoria_problema_saude = "Problema_Digestivo_Agudo".to_string();
        println!("  Inferência Nível 2: (Síndrome Gastrointestinal) -> Categoria_Problema_Saude = Problema Digestivo Agudo");
    } else if dor_localizacao == "peito" && dor_intensidade == "forte" { // Exemplo de regra com entrada direta para Nível 2
        categoria_problema_saude = "Condicao_Cardiaca_Potencial".to_string();
        println!("  Inferência Nível 2: (Dor no Peito Forte) -> Categoria_Problema_Saude = Condição Cardíaca Potencial");
    } else if dor_intensidade == "forte" && dor_localizacao == "cabeca" {
        categoria_problema_saude = "Cefaleia_Grave".to_string();
        println!("  Inferência Nível 2: (Dor de Cabeça Forte) -> Categoria_Problema_Saude = Cefaleia Grave");
    }
    fatos_conhecidos.insert("Categoria_Problema_Saude".to_string(), categoria_problema_saude.clone());


    // Nivel_Urgencia_Sistema (Este é crucial para a recomendação final)
    let mut nivel_urgencia_sistema = "Baixo_Autocuidado".to_string(); // Default

    // Ordem de Regras para Resolução de Conflitos: Prioridade de Urgência
    if indicador_alerta_grave == "sim" || categoria_problema_saude == "Condicao_Cardiaca_Potencial" {
        nivel_urgencia_sistema = "Extremo_Emergencia".to_string();
        println!("  Inferência Nível 2: (Sinal de Alerta Grave OU Condição Cardíaca Potencial) -> Nivel_Urgencia_Sistema = Extremo_Emergencia");
    } else if indicador_febre_alta == "sim" || categoria_problema_saude == "Infeccao_Respiratoria_Grave" || categoria_problema_saude == "Cefaleia_Grave" {
        nivel_urgencia_sistema = "Alto_Urgente".to_string();
        println!("  Inferência Nível 2: (Febre Alta OU Infecção Respiratória Grave OU Cefaleia Grave) -> Nivel_Urgencia_Sistema = Alto_Urgente");
    } else if sindrome_respiratoria_suspeita == "sim" || sindrome_gastrointestinal_suspeita == "sim" || indicador_dor_aguda == "sim" {
        nivel_urgencia_sistema = "Moderado_Agendamento".to_string();
        println!("  Inferência Nível 2: (Síndrome Respiratória OU Gastrointestinal OU Dor Aguda) -> Nivel_Urgencia_Sistema = Moderado_Agendamento");
    }
    // Se nenhuma das acima, permanece Baixo_Autocuidado.
    fatos_conhecidos.insert("Nivel_Urgencia_Sistema".to_string(), nivel_urgencia_sistema.clone());


    println!("\n--- Inferências Nível 2 Concluídas ---");

    // --- VARIÁVEIS DE OBJETIVO (SAÍDA FINAL) ---

    let mut recomendacao_atendimento = "Não foi possível determinar uma condição específica. Se os sintomas persistirem, procure um clínico geral.".to_string();
    let mut orientacao_adicional = "".to_string();

    // Regras Finais de Recomendação Baseadas no Nível de Urgência e Categoria
    match nivel_urgencia_sistema.as_str() {
        "Extremo_Emergencia" => {
            recomendacao_atendimento = "Procure atendimento de emergência IMEDIATAMENTE.".to_string();
            orientacao_adicional = "Não hesite, chame uma ambulância ou peça para alguém te levar ao pronto-socorro. Não dirija. Relate todos os seus sintomas.".to_string();
        },
        "Alto_Urgente" => {
            recomendacao_atendimento = "Agende uma consulta urgente com um clínico geral nas próximas 24h.".to_string();
            if categoria_problema_saude == "Infeccao_Respiratoria_Grave" {
                orientacao_adicional = "Monitore a falta de ar e a febre. Se piorar, procure emergência.".to_string();
            } else if categoria_problema_saude == "Cefaleia_Grave" {
                orientacao_adicional = "Descanse em ambiente tranquilo. Evite esforço. Procure médico urgentemente.".to_string();
            } else {
                orientacao_adicional = "Monitore seus sintomas de perto. Se houver piora súbita, vá à emergência.".to_string();
            }
        },
        "Moderado_Agendamento" => {
            let especialidade_provavel = match categoria_problema_saude.as_str() {
                "Infeccao_Respiratoria_Leve" => "Otorrinolaringologista ou Clínico Geral",
                "Problema_Digestivo_Agudo" => "Gastroenterologista ou Clínico Geral",
                _ => "Clínico Geral",
            };
            recomendacao_atendimento = format!("Agende uma consulta de rotina com um {}.", especialidade_provavel);
            orientacao_adicional = "Descanse, hidrate-se e evite a automedicação sem orientação. Marque sua consulta nos próximos dias.".to_string();
        },
        "Baixo_Autocuidado" => {
            recomendacao_atendimento = "Seus sintomas parecem leves. Repouso e hidratação devem ser suficientes.".to_string();
            orientacao_adicional = "Monitore seus sintomas. Se houver qualquer piora ou surgirem novos sintomas, procure um médico.".to_string();
        },
        _ => {
            // Caso catch-all para Nivel_Urgencia_Sistema não mapeado
        }
    }


    println!("\n--- Recomendação Final do Sistema ---");
    println!("Para {}:", nome);
    println!("Recomendação: {}", recomendacao_atendimento);
    if !orientacao_adicional.is_empty() {
        println!("Orientação Adicional: {}", orientacao_adicional);
    }
    println!("------------------------------------");

    println!("\n--- Explicação do Raciocínio ---");
    println!("O sistema coletou os seguintes sintomas e informações:");
    println!("  Temperatura Corporal: {}", fatos_conhecidos.get("Temperatura_Corporal").unwrap_or(&"N/A".to_string()));
    println!("  Dor Localização: {}", fatos_conhecidos.get("Dor_Localizacao").unwrap_or(&"N/A".to_string()));
    println!("  Dor Intensidade: {}", fatos_conhecidos.get("Dor_Intensidade").unwrap_or(&"N/A".to_string()));
    println!("  Sintomas Respiratórios: {}", fatos_conhecidos.get("Sintomas_Respiratorios").unwrap_or(&"N/A".to_string()));
    println!("  Sintomas Gastrointestinais: {}", fatos_conhecidos.get("Sintomas_Gastrointestinais").unwrap_or(&"N/A".to_string()));
    println!("  Sinais de Alerta Graves: {}", fatos_conhecidos.get("Sinais_Alerta_Graves").unwrap_or(&"N/A".to_string()));
    println!("  Histórico Médico Relevante: {}", fatos_conhecidos.get("Historico_Medico_Relevante").unwrap_or(&"N/A".to_string()));

    println!("\nCom base nesses dados, o sistema inferiu:");
    println!("  Inferências de Nível 1:");
    println!("    Indicador Febre Alta: {}", fatos_conhecidos.get("Indicador_Febre_Alta").unwrap_or(&"N/A".to_string()));
    println!("    Sindrome Respiratória Suspeita: {}", fatos_conhecidos.get("Sindrome_Respiratoria_Suspeita").unwrap_or(&"N/A".to_string()));
    println!("    Sindrome Gastrointestinal Suspeita: {}", fatos_conhecidos.get("Sindrome_Gastrointestinal_Suspeita").unwrap_or(&"N/A".to_string()));
    println!("    Indicador Dor Aguda: {}", fatos_conhecidos.get("Indicador_Dor_Aguda").unwrap_or(&"N/A".to_string()));
    println!("    Indicador Alerta Grave: {}", fatos_conhecidos.get("Indicador_Alerta_Grave").unwrap_or(&"N/A".to_string()));

    println!("\n  Inferências de Nível 2:");
    println!("    Categoria Problema Saúde: {}", fatos_conhecidos.get("Categoria_Problema_Saude").unwrap_or(&"N/A".to_string()));
    println!("    Nível de Urgência do Sistema: {}", fatos_conhecidos.get("Nivel_Urgencia_Sistema").unwrap_or(&"N/A".to_string()));

    println!("\nA recomendação foi gerada com base no Nível de Urgência do Sistema e na Categoria do Problema de Saúde inferida.");
    println!("---------------------------------");
}