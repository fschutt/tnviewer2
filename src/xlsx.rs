
use crate::csv::CsvDataType;

pub fn generate_report(datensaetze: &CsvDataType) -> Vec<u8> {

    use simple_excel_writer::*;
    
    let mut wb = Workbook::create_in_memory();
    let mut sheet = wb.create_sheet("Preferences");

    // ID
    sheet.add_column(Column { width: 50.0 });
    // Status
    sheet.add_column(Column { width: 500.0 });
    // Notiz
    sheet.add_column(Column { width: 500.0 });
    // Eigentümer
    sheet.add_column(Column { width: 20.0 });

        let _ = wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;
            sw.append_row(row!["ID", "Status", "Notiz", "Eigentümer"])?;
            for (flst_id, ds) in datensaetze.iter() {
                let ds_0 = match ds.get(0) {
                    Some(s) => s,
                    None => continue
                };
                let notiz = ds_0.notiz.clone();
                let status = ds_0.status.clone();
                let mut eigentuemer = ds.iter().map(|s| s.eigentuemer.clone()).collect::<Vec<_>>();
                eigentuemer.sort();
                eigentuemer.dedup();
                let eig = eigentuemer.join("; ");
                let s = match status {
                    crate::csv::Status::Bleibt => "bleibt",
                    crate::csv::Status::AenderungKeineBenachrichtigung => "Nutzungfl. ändern (keine Benachrichtigung)",
                    crate::csv::Status::AenderungMitBenachrichtigung => "Änderung mit Benachrichtigung",
                };
                sw.append_row(row![
                    flst_id.to_string(), 
                    s.to_string(),
                    notiz.to_string(),
                    eig.to_string()
                ])?;
            }

            Ok(())
        });

    match wb.close() {
        Ok(Some(o)) => o,
        _ => Vec::new(),
    }
}
